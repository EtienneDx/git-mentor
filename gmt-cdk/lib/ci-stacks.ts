import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as origins from 'aws-cdk-lib/aws-cloudfront-origins';
import * as cwlogs from 'aws-cdk-lib/aws-logs';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as cw from 'aws-cdk-lib/aws-cloudwatch';
import * as cwactions from 'aws-cdk-lib/aws-cloudwatch-actions';
import * as s3deploy from 'aws-cdk-lib/aws-s3-deployment';
import * as cd from 'aws-cdk-lib/aws-codedeploy';

interface CiStackProps extends cdk.StackProps {
  pullRequestId: string;
}

export class CiStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: CiStackProps) {
    super(scope, id, props);

    // frontend S3 bucket
    let bucket = new s3.Bucket(this, 'Frontend Bucket', {
      bucketName: `gmt-pull-ci-frontend-bucket-${props.pullRequestId}`,
      versioned: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      publicReadAccess: true,
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ACLS,
      accessControl: s3.BucketAccessControl.BUCKET_OWNER_FULL_CONTROL,
      websiteIndexDocument: 'index.html',
      websiteErrorDocument: 'index.html',
    });

    // artefacts S3 bucket
    let artefactsBucket = new s3.Bucket(this, 'Artefacts Bucket', {
      bucketName: `gmt-pull-ci-artefacts-bucket-${props.pullRequestId}`,
      versioned: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      publicReadAccess: false,
    });

    // Cloudwatch group
    let group = new cwlogs.LogGroup(this, 'Frontend Log Group', {
      logGroupName: `/git-mentor/pulls/${props.pullRequestId}`,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    // IAM role for ec2 backend to access the bucket and group
    let role = new iam.Role(this, 'Frontend Role', {
      roleName: `gmt-pull-ci-role-${props.pullRequestId}`,
      assumedBy: new iam.ServicePrincipal('ec2.amazonaws.com'),
    });

    group.grantWrite(role);
    artefactsBucket.grantReadWrite(role);

    // VPC
    let vpc = new ec2.Vpc(this, 'Backend VPC', {
      maxAzs: 1,
      subnetConfiguration: [
        {
          cidrMask: 24,
          name: 'Public',
          subnetType: ec2.SubnetType.PUBLIC,
        },
      ],
    });

    // ec2 instance
    let instance = new ec2.Instance(this, 'Backend Instance', {
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T2, ec2.InstanceSize.MICRO),
      machineImage: new ec2.AmazonLinuxImage({
        generation: ec2.AmazonLinuxGeneration.AMAZON_LINUX_2,
      }),
      associatePublicIpAddress: true,
      vpc: vpc,
      role: role,
      init: ec2.CloudFormationInit.fromElements(
        // Install cloudwatch log agent
        ec2.InitPackage.yum('amazon-cloudwatch-agent'),
        // Configure cloudwatch log agent
        ec2.InitCommand.shellCommand(`cat <<EOF > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json
        {
          "logs": {
            "logs_collected": {
              "files": {
                "collect_list": [
                  {
                    "file_path": "/var/log/gmt-server.log",
                    "log_group_name": "/git-mentor/pulls/${props.pullRequestId}",
                    "log_stream_name": "{instance_id}"
                  },
                  {
                    "file_path": "/var/log/gmt-api.log",
                    "log_group_name": "/git-mentor/pulls/${props.pullRequestId}",
                    "log_stream_name": "{instance_id}"
                  }
                ]
              }
            }
          }
        }`),
        // Start cloudwatch log agent
        ec2.InitCommand.shellCommand('amazon-cloudwatch-agent-ctl -a start'),
        // Install codedeploy agent
        ec2.InitPackage.yum('ruby'),
        ec2.InitPackage.yum('wget'),
        ec2.InitCommand.shellCommand('cd /home/ec2-user'),
        ec2.InitCommand.shellCommand('wget https://aws-codedeploy-eu-west-3.s3.eu-west-3.amazonaws.com/latest/install'),
        ec2.InitCommand.shellCommand('chmod +x ./install'),
        ec2.InitCommand.shellCommand('./install auto'),
        // Install PostgreSQL
        ec2.InitCommand.shellCommand('amazon-linux-extras install postgresql14'),
        ec2.InitCommand.shellCommand('yum install -y postgresql-server'),
        // Initialize PostgreSQL
        ec2.InitCommand.shellCommand('postgresql-setup initdb'),
        // Start PostgreSQL
        ec2.InitCommand.shellCommand('systemctl start postgresql'),
        // Create database
        ec2.InitCommand.shellCommand('sudo -u postgres psql -c "CREATE DATABASE gmt"'),
      ),
    });
    cdk.Tags.of(instance).add('GMT-CI', `pull-${props.pullRequestId}`);

    // code deploy application
    let apiApp = new cd.ServerApplication(this, 'API Application', {
      applicationName: `gmt-pull-${props.pullRequestId}-api-application`,
    });
    let gitApp = new cd.ServerApplication(this, 'Git Application', {
      applicationName: `gmt-pull-${props.pullRequestId}-git-application`,
    });

    // code deploy group
    let apiGroup = new cd.ServerDeploymentGroup(this, 'API Deployment Group', {
      application: apiApp,
      deploymentGroupName: `gmt-pull-${props.pullRequestId}-api-deployment-group`,
      ec2InstanceTags: new cd.InstanceTagSet({
        'GMT-CI': [`pull-${props.pullRequestId}`],
      }),
    });
    let gitGroup = new cd.ServerDeploymentGroup(this, 'Git Deployment Group', {
      application: gitApp,
      deploymentGroupName: `gmt-pull-${props.pullRequestId}-git-deployment-group`,
      ec2InstanceTags: new cd.InstanceTagSet({
        'GMT-CI': [`pull-${props.pullRequestId}`],
      }),
    });

    // Cloudfront distribution
    let distribution = new cloudfront.Distribution(this, 'Frontend Distribution', {
      defaultBehavior: { 
        origin: new origins.S3Origin(bucket, {
          originShieldEnabled: false,
        }),
      },
      additionalBehaviors: {
        '/api/*': {
          origin: new origins.HttpOrigin(instance.instancePublicDnsName),
          cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
          allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
        },
      },
    });

    // Open port ssh and http
    instance.connections.allowFromAnyIpv4(ec2.Port.tcp(22));
    instance.connections.allowFromAnyIpv4(ec2.Port.tcp(80));
    instance.connections.allowFromAnyIpv4(ec2.Port.tcp(443));

    // Create alarm on instance running more than 30 minutes to automatically stop it
    let metric = new cw.Metric({
      namespace: 'AWS/EC2',
      metricName: 'CPUUtilization',
      dimensionsMap: {
        InstanceId: instance.instanceId,
      },
      period: cdk.Duration.minutes(5),
    });
    let alarm = new cw.Alarm(this, 'Backend Alarm', {
      metric: metric,
      threshold: 0,
      evaluationPeriods: 6,
      comparisonOperator: cw.ComparisonOperator.GREATER_THAN_THRESHOLD,
    });

    alarm.addAlarmAction(new cwactions.Ec2Action(cwactions.Ec2InstanceAction.STOP));

    // Deployments

    //  - Upload frontend to S3
    new s3deploy.BucketDeployment(this, 'Deploy Frontend', {
      sources: [s3deploy.Source.asset('../gmt-web-app/build')],
      destinationBucket: bucket,
      distribution,
    });

    // Outputs
    new cdk.CfnOutput(this, 'DistributionDomainName', {
      value: distribution.distributionDomainName,
    });

    new cdk.CfnOutput(this, 'InstanceUrl', {
      value: instance.instancePublicDnsName,
    });

    new cdk.CfnOutput(this, 'InstanceId', {
      value: instance.instanceId,
    });
  }
}