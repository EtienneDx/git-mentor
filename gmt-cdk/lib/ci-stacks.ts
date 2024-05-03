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

interface CiStackProps extends cdk.StackProps {
  pullRequestId: number;
}

export class CiStackFront extends cdk.Stack {
  constructor(scope: Construct, id: string, props: CiStackProps) {
    super(scope, id, props);

    // frontend S3 bucket
    let bucket = new s3.Bucket(this, 'Frontend Bucket', {
      bucketName: `gmt-pull-ci-frontend-bucket-${props.pullRequestId}`,
      versioned: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      publicReadAccess: true,
    });

    // artefacts S3 bucket
    let artefactsBucket = new s3.Bucket(this, 'Artefacts Bucket', {
      bucketName: `gmt-pull-ci-artefacts-bucket-${props.pullRequestId}`,
      versioned: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      publicReadAccess: false,
    });

    // Cloudfront distribution
    let distribution = new cloudfront.Distribution(this, 'Frontend Distribution', {
      defaultBehavior: { 
        origin: new origins.S3Origin(bucket, {
          originShieldEnabled: false,
        }),
      },
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

    // Outputs
    new cdk.CfnOutput(this, 'DistributionDomainName', {
      value: distribution.distributionDomainName,
    });

    // Deployments will be made externally
  }
}

export class CiStackBack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: CiStackProps) {
    super(scope, id, props);

    // Retrieve role
    let role = iam.Role.fromRoleArn(this, 'Backend Role', `arn:aws:iam::${this.account}:role/gmt-pull-ci-role-${props.pullRequestId}`);

    // Retrieve group
    let group = cwlogs.LogGroup.fromLogGroupName(this, 'Backend Log Group', `/git-mentor/pulls/${props.pullRequestId}`);

    // VPC
    let vpc = new ec2.Vpc(this, 'Backend VPC', {
      maxAzs: 2,
    });

    // ec2 instance
    let instance = new ec2.Instance(this, 'Backend Instance', {
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T2, ec2.InstanceSize.MICRO),
      machineImage: new ec2.AmazonLinuxImage(),
      vpc: vpc,
      role: role,
      init: ec2.CloudFormationInit.fromElements(
        // Install cloudwatch log agent
        ec2.InitPackage.yum('awslogs'),
        // Configure cloudwatch log agent
        ec2.InitCommand.shellCommand('cat <<EOF > /etc/awslogs/awslogs.conf\n' +
          '[general]\n' +
          'state_file = /var/lib/awslogs/agent-state\n' +
          '\n' +
          '[/var/log/messages]\n' +
          'log_group_name = ' + group.logGroupName + '\n' +
          'log_stream_name = {instance_id}\n' +
          'datetime_format = %b %d %H:%M:%S\n' +
          'EOF\n'),
        // Start cloudwatch log agent
        ec2.InitCommand.shellCommand('systemctl start awslogsd'),
        // download artefacts from S3
        ec2.InitCommand.shellCommand('aws s3 cp s3://gmt-pull-ci-artefacts-bucket-${props.pullRequestId} /home/ec2-user/ --recursive'),
        // Install PostgreSQL
        ec2.InitPackage.yum('postgresql-server'),
        // Start PostgreSQL
        ec2.InitCommand.shellCommand('service postgresql start'),
        // Create database
        ec2.InitCommand.shellCommand('sudo -u postgres psql -c "CREATE DATABASE gmt"'),
        // Start backend api server
        ec2.InitCommand.shellCommand('cd /home/ec2-user && ./gmt-api'),
        // Start gmt git server
        ec2.InitCommand.shellCommand('cd /home/ec2-user && ./gmt-server'),
      ),
    });

    // Open port ssh and http
    instance.connections.allowFromAnyIpv4(ec2.Port.tcp(22));
    instance.connections.allowFromAnyIpv4(ec2.Port.tcp(80));

    // Create alarm on instance running more than 30 minutes to automatically stop it
    let metric = new cw.Metric({
      namespace: 'AWS/EC2',
      metricName: 'CPUUtilization',
      dimensionsMap: {
        InstanceId: instance.instanceId,
      },
      period: cdk.Duration.minutes(30),
    });
    let alarm = new cw.Alarm(this, 'Backend Alarm', {
      metric: metric,
      threshold: 0,
      evaluationPeriods: 1,
      comparisonOperator: cw.ComparisonOperator.GREATER_THAN_THRESHOLD,
    });

    alarm.addAlarmAction(new cwactions.Ec2Action(cwactions.Ec2InstanceAction.STOP));

    // Outputs
    new cdk.CfnOutput(this, 'InstanceUrl', {
      value: instance.instancePublicDnsName,
    });
  }
}