import { RemovalPolicy } from "aws-cdk-lib";
import { CloudFrontWebDistribution, Distribution, DistributionProps, ViewerProtocolPolicy } from "aws-cdk-lib/aws-cloudfront";
import { S3Origin, S3OriginProps } from "aws-cdk-lib/aws-cloudfront-origins";
import { BlockPublicAccess, Bucket, BucketAccessControl, BucketProps } from "aws-cdk-lib/aws-s3";
import { BucketDeployment, BucketDeploymentProps, Source } from "aws-cdk-lib/aws-s3-deployment";
import { Construct } from "constructs";

interface WebsiteBucketProps {
  bucketName: string;
  deployedAssets: string[];
  bucketProps?: BucketProps;
  bucketDeploymentProps?: BucketDeploymentProps;
  s3OriginProps?: S3OriginProps;
  distributionProps?: Partial<DistributionProps>;
}

export default class WebsiteBucket extends Construct {
  bucket: Bucket;
  distribution: Distribution;
  props: WebsiteBucketProps;

  constructor(scope: Construct, id: string, props: WebsiteBucketProps) {
    super(scope, id);
    this.props = props;

    // S3 bucket
    this.bucket = new Bucket(this, 'Frontend Bucket', {
      bucketName: props.bucketName,
      versioned: true,
      removalPolicy: RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      publicReadAccess: true,
      blockPublicAccess: BlockPublicAccess.BLOCK_ACLS,
      accessControl: BucketAccessControl.BUCKET_OWNER_FULL_CONTROL,
      websiteIndexDocument: 'index.html',
      websiteErrorDocument: 'index.html',
      ...props.bucketProps,
    });
    
    this.distribution = new Distribution(this, 'Frontend Distribution', {
      defaultBehavior: { 
        origin: this.s3_origin,
        viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
      },
      ...props.distributionProps,
    });

    new BucketDeployment(this, 'Deploy Frontend', {
      sources: props.deployedAssets.map(asset => Source.asset(asset)),
      destinationBucket: this.bucket,
      distribution: this.distribution,
      ...props.bucketDeploymentProps,
    });
  }

  get s3_origin() {
    return new S3Origin(this.bucket, this.props.s3OriginProps);
  }
}