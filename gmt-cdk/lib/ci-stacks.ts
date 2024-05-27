import { StackProps, Stack, RemovalPolicy, Tags, CfnOutput } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Bucket, BlockPublicAccess, BucketAccessControl } from 'aws-cdk-lib/aws-s3';
import { Distribution, CachePolicy, AllowedMethods, OriginProtocolPolicy } from 'aws-cdk-lib/aws-cloudfront';
import { S3Origin, HttpOrigin } from 'aws-cdk-lib/aws-cloudfront-origins';
import { LogGroup, RetentionDays } from 'aws-cdk-lib/aws-logs';
import { Role, ServicePrincipal } from 'aws-cdk-lib/aws-iam';
import { Vpc, SubnetType, Instance, InstanceType, InstanceClass, InstanceSize, AmazonLinuxImage, AmazonLinuxGeneration, CloudFormationInit, InitPackage, InitCommand, Port } from 'aws-cdk-lib/aws-ec2';
import { BucketDeployment, Source } from 'aws-cdk-lib/aws-s3-deployment';
import { ServerApplication, ServerDeploymentGroup, InstanceTagSet } from 'aws-cdk-lib/aws-codedeploy';
import WebsiteBucket from './constructs/website-bucket';
import ApplicationInstance from './constructs/application-instance';
import postgresInstall from './init-commands/postgres';
import cloudwatchAgent from './init-commands/cloudwatch-agent';
import codeDeployAgent from './init-commands/codedeploy-agent';

interface CiStackProps extends StackProps {
  pullRequestId: string;
}

export class CiStack extends Stack {
  constructor(scope: Construct, id: string, props: CiStackProps) {
    super(scope, id, props);

    // IAM role for ec2 backend to access the bucket and group
    let role = new Role(this, 'Frontend Role', {
      roleName: `gmt-pull-ci-role-${props.pullRequestId}`,
      assumedBy: new ServicePrincipal('ec2.amazonaws.com'),
    });

    // Cloudwatch group
    let group = new LogGroup(this, 'Frontend Log Group', {
      logGroupName: `/git-mentor/pulls/${props.pullRequestId}`,
      removalPolicy: RemovalPolicy.DESTROY,
      retention: RetentionDays.ONE_WEEK,
    });

    group.grantWrite(role);

    // Backend application
    let backend = new ApplicationInstance(this, 'Backend', {
      applicationName: `gmt-pull-ci-${props.pullRequestId}`,
      instanceRole: role,
      tags: {
        'GMT-CI': `pull-${props.pullRequestId}`,
      },
      openPorts: [Port.tcp(22), Port.tcp(2222), Port.tcp(80), Port.tcp(443)],
      instanceProps: {
        init: CloudFormationInit.fromElements(
          // Cloudwatch agent
          ...cloudwatchAgent({
            logs: {
              files: {
                collect_list: [
                  // gmt server
                  {
                    file_path: '/gmt/logs/gmt-server.log',
                    log_group_name: `/git-mentor/pulls/${props.pullRequestId}`,
                    log_stream_name: '{instance_id}/gmt-server',
                  },
                  // gmt api
                  {
                    file_path: '/gmt/logs/gmt-api.log',
                    log_group_name: `/git-mentor/pulls/${props.pullRequestId}`,
                    log_stream_name: '{instance_id}/gmt-api',
                  },
                  // codedeploy
                  {
                    file_path: '/var/log/aws/codedeploy-agent/codedeploy-agent.log',
                    log_group_name: `/git-mentor/pulls/${props.pullRequestId}`,
                    log_stream_name: '{instance_id}/codedeploy-agent',
                  },
                ],
              },
            },
          }),
          // CodeDeploy agent
          ...codeDeployAgent(),
          // Postgres
          ...postgresInstall({
            databaseName: 'gmt',
            databaseUser: 'admin_user',
            databasePassword: 'admin',
          }),
        ),
      }
    });

    let frontend = new WebsiteBucket(this, 'Frontend', {
      bucketName: `gmt-pull-ci-frontend-bucket-${props.pullRequestId}`,
      deployedAssets: ['../gmt-web-app/build', '../gmt-web-app/prod'],
      s3OriginProps: {
        originShieldEnabled: false,
      },
      distributionProps: {
        additionalBehaviors: {
          '/api/*': {
            origin: new HttpOrigin(backend.instance.instancePublicDnsName, {
              protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
            }),
            cachePolicy: CachePolicy.CACHING_DISABLED,
            allowedMethods: AllowedMethods.ALLOW_ALL,
          },
        },
      },
    });

    // Outputs
    new CfnOutput(this, 'DistributionDomainName', {
      value: frontend.distribution.distributionDomainName,
    });

    new CfnOutput(this, 'InstanceId', {
      value: backend.instance.instanceId,
    });

    new CfnOutput(this, 'BackendArtefactBucket', {
      value: backend.artefactsBucketName,
    });

    new CfnOutput(this, 'BackendApplicationName', {
      value: backend.application.applicationName,
    });

    new CfnOutput(this, 'BackendDeploymentGroupName', {
      value: backend.deploymentGroup.deploymentGroupName,
    });
  }
}