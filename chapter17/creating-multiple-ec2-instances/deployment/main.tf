# builds/main.tf
terraform {
    required_version = ">= 1.9.4"
    required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 5.62.0"
    }
  }
}

provider "aws" {
  region = "eu-west-2"
}

variable "db_password" {
    description = "The password for the database"
    default = "password"
}

variable "db_username" {
    description = "The username for the database"
    default = "username"
}

resource "aws_instance" "production_server" {
    ami = "ami-05ea2888c91c97ca7"
    instance_type = "t2.medium"
    count = 2
    key_name = "remotebuild"
    user_data = file("server_build.sh")
    tags = {
      Name = "to-do prod ${count.index}"
    }
}

# PostgreSQL Database
resource "aws_db_instance" "main_db" {
  instance_class         = "db.t3.micro"
  allocated_storage      = 5
  engine                 = "postgres"
  username               = var.db_username
  password               = var.db_password
  db_name                = "to_do"
  publicly_accessible    = true
  skip_final_snapshot    = true
  parameter_group_name   = aws_db_parameter_group.custom_pg.name
  tags = {
      Name = "to-do production database"
    }
}

resource "aws_db_parameter_group" "custom_pg" {
  name        = "custom-pg"
  family      = "postgres16"
  description = "allows unencrypted connections"

  parameter {
    name  = "rds.force_ssl"
    value = "0"
  }
}

# Outputs
output "ec2_global_ips" {
  value = ["${aws_instance.production_server.*.public_ip}"]
}

output "db_endpoint" {
  value = "${aws_db_instance.main_db.*.endpoint}"
}

output "public_dns" {
  value = ["${aws_instance.production_server.*.public_dns}"]
}

output "instance_id" {
  value = ["${aws_instance.production_server.*.id}"]
}