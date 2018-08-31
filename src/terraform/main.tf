data "aws_caller_identity" "current" {}

locals {
  project_name = "lazywrite"
}

terraform {
  backend "s3" {
    bucket = "richardanaya-remote-state"
    key    = "${local.project_name}/terraform.tfstate"
    region = "us-east-1"
  }
}

module "website-lambda" {
  source     = "../../.vendor/terraform/aws-lambda"
  name       = "${local.project_name}"
  zip_path  = "../lambdas/${local.project_name}.zip"
  policy_statements = <<EOF
{
    "Effect": "Allow",
    "Action": "secretsmanager:GetSecretValue",
    "Resource": "arn:aws:secretsmanager:us-east-1:${data.aws_caller_identity.current.account_id}:secret:elephantsql_connection-At46Hm"
}
EOF
}

module "website-gw" {
  source     = "../../.vendor/terraform/aws-api-gw-single-lambda"
  name       = "${local.project_name}"
  lambda-arn = "${module.website-lambda.arn}"
}
