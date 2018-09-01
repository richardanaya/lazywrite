# Project level variables
PROJECT_NAME         = lazywrite
PROJECT_DESCRIPTION  = A website for story making

# Tools
GIT                  = git
RUST_I               = richardanaya/aws-lambda-rust:1.28.0 cargo
CARGO                = docker run -v $(abspath .vendor/cargo):/home/.cargo -e CARGO_HOME='/home/.cargo' -v `pwd`:/code -w /code $(RUST_I)
#TERRAFORM            = docker run -v $(abspath .vendor/terraform):/home/.terraform.d -v $(PWD):/code -w /code/dist/terraform hashicorp/terraform:0.11.8
# Vendoring
ifneq ("$(wildcard .vendor)","")
include .vendor/make/prelude.mk
include .vendor/make/help.mk
include .vendor/make/lambda_rust.mk
include .vendor/make/terraform.mk
include .vendor/make/website.mk
endif

.PHONY : all check deploy clean

##all          - Build everything
all            : lambda_rust__build terraform__build website__build
	#We need to include this library with our lambda so diesel works
	@zip -9 -j dist/lambdas/$(PROJECT_NAME).zip src/libpq.so.5

##clean        - Clean up project
clean          : lambda_rust__clean terraform__clean website__clean

##check-deploy - Verify next deploy will succeed
check-deploy   : all terraform__plan

##deploy       - Deploy infrastructure
deploy         : all terraform__apply

##destroy      - Destroy infrastructure
destroy        : all terraform__destroy

##run          -
run            :
	@docker run -p 3030:3030 -v $(abspath .vendor/cargo):/home/.cargo -e CARGO_HOME='/home/.cargo' -v `pwd`:/code -w /code/src/lambdas/lazywrite $(RUST_I) run --release --features "local_development"

##vendor       - Vendor makefiles
vendor         :
	@echo Vendoring Makefiles
	@rm -rf .vendor
	@$(GIT) clone https://github.com/richardanaya/makefiles.git .vendor/make
	@$(GIT) clone https://github.com/richardanaya/terraforms.git .vendor/terraform
	@mkdir -p .vendor/cargo
	@yarn
