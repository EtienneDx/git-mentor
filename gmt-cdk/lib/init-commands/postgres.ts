import { InitCommand, InitElement } from "aws-cdk-lib/aws-ec2";

export interface PostgresInitCommandProps {
  databaseName: string;
  databaseUser: string;
  databasePassword: string;
}

export default function postgresInstall(props: PostgresInitCommandProps): InitElement[] {
  return [
    // Install PostgreSQL
    InitCommand.shellCommand('dnf install -y postgresql15 postgresql15-server libpq'),
    // Initialize PostgreSQL
    InitCommand.shellCommand('postgresql-setup initdb'),
    // Start PostgreSQL
    InitCommand.shellCommand('systemctl start postgresql'),
    // Create user
    InitCommand.shellCommand(`sudo -u postgres psql -c "CREATE USER ${props.databaseUser} WITH PASSWORD \'${props.databasePassword}\'"`),
    // Create database
    InitCommand.shellCommand(`sudo -u postgres psql -c "CREATE DATABASE ${props.databaseName}"`),
    // Allow user to access database
    InitCommand.shellCommand(`sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE ${props.databaseName} TO ${props.databaseUser}"`),
    InitCommand.shellCommand(`sudo -u postgres psql -c "GRANT ALL ON SCHEMA public TO ${props.databaseUser}"`),
    InitCommand.shellCommand(`sudo -u postgres psql -c "ALTER DATABASE gmt OWNER TO ${props.databaseUser}"`),
    // Allow usage of password
    InitCommand.shellCommand(`sed -i 's/ident/md5/g' /var/lib/pgsql/data/pg_hba.conf`),
    // Restart PostgreSQL
    InitCommand.shellCommand('systemctl restart postgresql'),
  ]
}