import { RemovalPolicy, Tags } from "aws-cdk-lib";
import { InstanceTagSet, ServerApplication, ServerDeploymentGroup } from "aws-cdk-lib/aws-codedeploy";
import { AmazonLinuxGeneration, AmazonLinuxImage, Instance, InstanceClass, InstanceProps, InstanceSize, InstanceType, Port, SubnetType, Vpc, VpcProps } from "aws-cdk-lib/aws-ec2";
import { Role } from "aws-cdk-lib/aws-iam";
import { Bucket } from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";

interface ApplicationInstanceProps {
  applicationName: string;
  instanceRole: Role;
  vpcProps?: VpcProps;
  openPorts: Port[];
  instanceProps: Partial<InstanceProps>;
  tags: { [key: string]: string };
}

export default class ApplicationInstance extends Construct {
  artefactsBucket: Bucket;
  instance: Instance;
  vpc: Vpc;
  application: ServerApplication;
  deploymentGroup: ServerDeploymentGroup;

  constructor(scope: Construct, id: string, props: ApplicationInstanceProps) {
    super(scope, id);

    // artefacts S3 bucket
    this.artefactsBucket = new Bucket(this, 'Artefacts Bucket', {
      bucketName: `${props.applicationName}-artefacts-bucket`,
      versioned: true,
      removalPolicy: RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      publicReadAccess: false,
    });
    
    this.artefactsBucket.grantReadWrite(props.instanceRole);

    // VPC
    this.vpc = new Vpc(this, 'Backend VPC', {
      maxAzs: 1,
      subnetConfiguration: [
        {
          cidrMask: 24,
          name: 'Public',
          subnetType: SubnetType.PUBLIC,
        },
      ],
      ...props.vpcProps,
    });

    // ec2 instance
    this.instance = new Instance(this, 'Backend Instance', {
      instanceType: InstanceType.of(InstanceClass.T2, InstanceSize.MICRO),
      machineImage: new AmazonLinuxImage({
        generation: AmazonLinuxGeneration.AMAZON_LINUX_2023,
      }),
      associatePublicIpAddress: true,
      vpc: this.vpc,
      role: props.instanceRole,
      ...props.instanceProps,
    });
    for (let [key, value] of Object.entries(props.tags)) {
      Tags.of(this.instance).add(key, value);
    }

    // open ports
    props.openPorts.forEach(port => {
      this.instance.connections.allowFromAnyIpv4(port);
    });

    // code deploy application
    this.application = new ServerApplication(this, 'API Application', {
      applicationName: `${props.applicationName}-application`,
    });

    // reformat tags
    let tags: { [key: string]: string[] } = {};
    for (let [key, value] of Object.entries(props.tags)) {
      tags[key] = [value];
    }

    // code deploy group
    this.deploymentGroup = new ServerDeploymentGroup(this, 'Deployment Group', {
      application: this.application,
      deploymentGroupName: `${props.applicationName}-deployment-group`,
      ec2InstanceTags: new InstanceTagSet(tags),
    });
  }

  get artefactsBucketName() {
    return this.artefactsBucket.bucketName;
  }
}