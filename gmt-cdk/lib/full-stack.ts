import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as sqs from 'aws-cdk-lib/aws-sqs';
import * as s3 from 'aws-cdk-lib/aws-s3';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as codedeploy from 'aws-cdk-lib/aws-codedeploy';
import * as s3Deployment from 'aws-cdk-lib/aws-s3-deployment';

export class FullStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const queue = new sqs.Queue(this, 'GmtQueue', {
      queueName: 'GmtQueue',
      visibilityTimeout: cdk.Duration.seconds(300)
    });

    new cdk.CfnOutput(this, 'QueueUrl', {
      value: queue.queueUrl
    });

    this.createInstance();
    this.createFrontend();
  }

  createInstance() {
    const vpc = new ec2.Vpc(this, 'GmtVpc', {
      maxAzs: 2,
      natGateways: 1
    });

    const securityGroup = new ec2.SecurityGroup(this, 'GmtSecurityGroup', {
      vpc,
      allowAllOutbound: true
    });
    securityGroup.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(80), 'Allow HTTP access');
    securityGroup.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(22), 'Allow SSH access');

    const instance = new ec2.Instance(this, 'GmtInstance', {
      vpc,
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T2, ec2.InstanceSize.MICRO),
      machineImage: ec2.MachineImage.latestAmazonLinux2(),
      securityGroup,
    });
    cdk.Tags.of(instance).add('Application', 'GmtServer');

    const codeDeploy = new codedeploy.ServerApplication(this, 'GmtCodeDeployApp', {
      applicationName: 'GmtCodeDeployApp',
    });
    const codeDeployDeploymentGroup = new codedeploy.ServerDeploymentGroup(this, 'GmtCodeDeployDeploymentGroup', {
      application: codeDeploy,
      deploymentGroupName: 'GmtCodeDeployDeploymentGroup',
      ec2InstanceTags: new codedeploy.InstanceTagSet({
        Application: ['GmtServer']
      }),
    });

    instance.userData.addCommands(
      'sudo yum install -y ruby',
      'sudo yum install -y wget',
      'cd /home/ec2-user',
      'wget https://aws-codedeploy-us-east-1.s3.amazonaws.com/latest/install',
      'chmod +x ./install',
      'sudo ./install auto'
    );

    new cdk.CfnOutput(this, 'InstanceId', {
      value: instance.instanceId
    });
  }

  createFrontend() {
    const websiteBucket = new s3.Bucket(this, 'GmtWebsite', {
      bucketName: 'gmt-website',
      websiteIndexDocument: 'index.html',
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    const oai = new cloudfront.OriginAccessIdentity(this, 'OAI', {
      comment: 'OAI for GMT Website'
    });
    websiteBucket.grantRead(oai);

    const distribution = new cloudfront.CloudFrontWebDistribution(this, 'GmtWebsiteDistribution', {
      originConfigs: [
        {
          s3OriginSource: {
            s3BucketSource: websiteBucket,
            originAccessIdentity: oai,
          },
          behaviors: [{ isDefaultBehavior: true, }],
        }
      ],
      // we redirect the errors to the index, so that react can handle them
      errorConfigurations: [
        {
          errorCode: 403,
          responseCode: 200,
          responsePagePath: '/index.html',
          errorCachingMinTtl: 0,
        },
        {
          errorCode: 404,
          responseCode: 200,
          responsePagePath: '/index.html',
          errorCachingMinTtl: 0,
        }
      ],
    });

    new s3Deployment.BucketDeployment(this, 'DeployWebsite', {
      sources: [s3Deployment.Source.asset('../gmt-web-app/build')],
      destinationBucket: websiteBucket,
      distribution: distribution,
      distributionPaths: ['/*'],
    });

    new cdk.CfnOutput(this, 'WebsiteURL', {
      value: distribution.distributionDomainName
    });
  }
}