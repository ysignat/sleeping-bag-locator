# sleeping-bag-locator

This app was designed to track the location of my belongings

It was an easy task until I realized that items could be left thousands of kilometers apart from each other

Its name refers to the hours spent looking for my sleeping bag that I lent to my friend (and of course forgot whom and when I lent it to)

## Infrastructure setup

Despite the fact that all the infrastructure configuration is stored as code in the [deploy](./deploy/) and [infra](./infra/) subfolders, some manual actions are needed

As [Yandex Cloud](https://yandex.cloud/en) is gently granting me a small amount of money for personal projects, it is the main cloud provider for this project

### Step 1. Creating account and folder

- Create account - [docs](https://yandex.cloud/en-ru/docs/billing/quickstart/)
- Create folder - [docs](https://yandex.cloud/en-ru/docs/resource-manager/operations/folder/create)

I prefer to place infrastructure for different projects in separate cloud folders, to be able to take a look at their cost separately

After creating an account and folder you could download [yc](https://yandex.cloud/en-ru/docs/cli/) and create the profile for your project

```sh
yc config profile create sleeping-bag-locator
yc config set token ${OAUTH_TOKEN} # Could be obtained at https://oauth.yandex.com/authorize?response_type=token&client_id=1a6990aa636648e9b2ef855fa7bec2fb
yc config set cloud-id ${CLOUD_ID}
yc config set folder-id ${FOLDER_ID}
```

### Step 2. Creating service account and static keys

To manage infrastructure you need a service account and its static keys

A service account can be created via

```sh
yc iam service-account create --name sleeping-bag-locator-terraform --description 'SA for terraform-related actions'
```

Since you wish to manage all of the infrastructure in your project with this service account, you have to give it a `Folder - Admin` role.

```sh
yc resource-manager folder add-access-binding --id $(yc config get folder-id) --role admin --service-account-name sleeping-bag-locator-terraform
```

Also you need static keys for this service account to use it in services with AWS-compatible API

```sh
yc iam access-key create --service-account-name sleeping-bag-locator-terraform
```

> Save the `ACCESS_KEY` (`access_key.id`) and `SECRET_KEY` (`secret`). You will not be able to get their values again

### Step 3. Creating an S3 bucket and YDB serverless database for Terraform

We need an S3 bucket for storing states

```sh
yc storage bucket create --name sleeping-bag-locator-terraform --max-size 1073741824 --default-storage-class ICE
```

> Don't forget to enable versioning in the bucket (this can be made only via the management console)

Also we need to create a YDB database for locking states via DynamoDB API and a table in it for the state locks storage

```sh
yc ydb database create --name terraform-state-locks --serverless --description 'DB for terraform state locks management' --sls-enable-throttling-rcu --sls-throttling-rcu 3 --sls-storage-size 1 --deletion-protection

aws dynamodb create-table --table-name state-lock-table --attribute-definitions AttributeName=LockID,AttributeType=S --key-schema AttributeName=LockID,KeyType=HASH --endpoint $(yc ydb database get terraform-state-locks --format json | jq -r '.document_api_endpoint') --region $(yc ydb database get terraform-state-locks --format json | jq -r '.location_id')
```

### Step 4. Create token for Github management

To run CD pipelines for infrastructure changes you should create a token [here](https://github.com/settings/tokens) with all scopes present

After that put it into `GITHUB_TOKEN` variable in `.env` and into `GH_TERRAFORM_TOKEN` variable in [Github Actions secrets](https://github.com/ysignat/sleeping-bag-locator/settings/secrets/actions)
