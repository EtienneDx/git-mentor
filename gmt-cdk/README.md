# Gmt Cdk

This project contains the infrastructure definition for the GMT project. It provides acces to 2 stacks, one holding the
required resources for development only and one holding the full infrastructure to deploy the project. This separation is
likely to evolve, for example to provide a staging environment. We might also decide to move to a managed service for which
all the infrastructure will be managed by us and not by the client.

## Development stack

The development stack is defined in the `lib/dev-stack.ts` file. It contains the following resources:

- GmtQueue-dev: A SQS queue used to manage the CI jobs

## Production stack

The production stack is defined in the `lib/full-stack.ts` file. It contains the following resources:

- GmtQueue: A SQS queue used to manage the CI jobs
- GmtVpc: The VPC used by the GmtInstance
- GmtInstance: The EC2 instance used to hold the different GMT services. The instance contains the code deploy agent which allow for the deployment of the services. The instance is tagged [Application -> GmtServer] for the code deploy agent to be able to identify it.
- GmtSecurityGroup: The security group used by the GmtInstance
- GmtCodeDeployApp: The CodeDeploy application used to deploy the GMT services
- GmtCodeDeployDeploymentGroup: The CodeDeploy deployment group used to deploy the GMT services
- GmtWebsite: The S3 bucket used to host the GMT front-end website
- GmtWebsiteDistribution: The CloudFront distribution used to serve the GMT front-end website. It redirects all errors to the index.html file to allow for the front-end routing to work. It uses OAI as CDK does not support OAC yet.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `cdk deploy`      deploy this stack to your default AWS account/region
* `cdk diff`        compare deployed stack with current state
* `cdk synth`       emits the synthesized CloudFormation template
