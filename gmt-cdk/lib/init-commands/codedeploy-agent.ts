import { InitCommand, InitElement, InitPackage } from "aws-cdk-lib/aws-ec2";

interface CodeDeployProps {
  user?: string;
}

export default function codeDeployAgent(props ?: CodeDeployProps): InitElement[] {
  return [
    // Install codedeploy agent
    InitCommand.shellCommand('yum update -y'),
    InitCommand.shellCommand('yum install -y ruby'),
    InitPackage.yum('wget'),
    InitCommand.shellCommand(`cd /home/${props?.user ?? 'ec2-user'}`),
    InitCommand.shellCommand('wget https://aws-codedeploy-eu-west-3.s3.eu-west-3.amazonaws.com/latest/install'),
    InitCommand.shellCommand('chmod +x ./install'),
    InitCommand.shellCommand('./install auto'),
    // Cleanup
    InitCommand.shellCommand('rm -f ./install'),
  ]
}