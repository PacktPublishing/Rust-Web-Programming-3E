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

resource "aws_instance" "build_server" {
    ami = "ami-05ea2888c91c97ca7"
    instance_type = "t2.medium"
    key_name = "remotebuild"
    user_data = file("server_build.sh")
    tags = {
    Name = "to-do build server"
    }

    # root disk
    root_block_device {
      volume_size = "150"
      volume_type = "gp2"
      delete_on_termination = true
    }
}

output "ec2_global_ips" {
    value = ["${aws_instance.build_server.*.public_ip}"]
}
