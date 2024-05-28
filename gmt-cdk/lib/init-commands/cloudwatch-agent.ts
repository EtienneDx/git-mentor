import { InitCommand, InitElement, InitPackage } from "aws-cdk-lib/aws-ec2";

interface CloudWatchConfig {
  logs: {
    files: {
      collect_list: {
        file_path: string;
        log_group_name: string;
        log_stream_name: string;
      }[];
    };
  };
}

export default function cloudwatchAgent(config: CloudWatchConfig): InitElement[] {
  return [
    // Install cloudwatch log agent
    InitPackage.yum('amazon-cloudwatch-agent'),
    // Configure cloudwatch log agent
    InitCommand.shellCommand(`cat <<EOF > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json
    ${JSON.stringify(config, null, 2)}`),
    // Start cloudwatch log agent
    InitCommand.shellCommand('amazon-cloudwatch-agent-ctl -a start'),
  ]
}