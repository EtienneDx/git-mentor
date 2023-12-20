#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { DevStack } from '../lib/dev-stack';
import { FullStack } from '../lib/full-stack';

const app = new cdk.App();
new DevStack(app, 'GmtCdkStack-dev', {
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: "eu-west-3" },
});
new FullStack(app, 'GmtCdkStack', {
  env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: "eu-west-3" },
});