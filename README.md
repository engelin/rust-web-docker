## Rust Web Server Example with Actix-web

### 1. Getting Started
1.1 Build the Docker image
```bash
$ docker build -t engelin/rust-web-docker .
[+] Building 125.5s (13/13) FINISHED                                                                                                                                                                           
 => [internal] load build definition from Dockerfile                                                                                                                                                      0.0s
 => => transferring dockerfile: 409B                                                                                                                                                                      0.0s
 => [internal] load .dockerignore                                                                                                                                                                         0.0s
 => => transferring context: 34B                                                                                                                                                                          0.0s
 => [internal] load metadata for docker.io/library/debian:buster-slim                                                                                                                                     6.2s
 => [internal] load metadata for docker.io/library/rust:latest                                                                                                                                            7.0s
 => [builder 1/4] FROM docker.io/library/rust@sha256:8a4ca3ca75afbc97bcf5362e9a694fe049d15734fbbaf82b8b7e224616c1254b                                                                                    57.0s
 => => resolve docker.io/library/rust@sha256:8a4ca3ca75afbc97bcf5362e9a694fe049d15734fbbaf82b8b7e224616c1254b                                                                                             0.0s
 => => sha256:17f8f0dab8e15df1275d0645dab39dd1077b273deb44ee6c8fea64acdae84e5b 1.38kB / 1.38kB                                                                                                            0.0s
 => => sha256:fa9d1522979dada89205a0a6d8ab2971393cb63bbe3cc86ee90b8b369bed3884 6.11kB / 6.11kB                                                                                                            0.0s
 => => sha256:fa34261389f3dee2ecffe9bfe38ae7fd05a1908e98a7486354905aee0f648a6e 49.59MB / 49.59MB                                                                                                         18.3s
 => => sha256:993525952a483617183655b735562fd3eee4b3a85a472e3dc3c79a258adca100 23.57MB / 23.57MB                                                                                                          5.1s
 => => sha256:12916f73f9de8e251c4eb4f2d079fa6cd63205b7ba8a7bd88e1f9446105e9ef1 63.99MB / 63.99MB                                                                                                         20.6s
 => => sha256:8a4ca3ca75afbc97bcf5362e9a694fe049d15734fbbaf82b8b7e224616c1254b 988B / 988B                                                                                                                0.0s
 => => sha256:371506b77bb850df2c7768a7e0e38b6259d82a0e6cd2d287800c151aa7d771c4 202.39MB / 202.39MB                                                                                                       40.4s
 => => extracting sha256:fa34261389f3dee2ecffe9bfe38ae7fd05a1908e98a7486354905aee0f648a6e                                                                                                                 0.8s
 => => sha256:39ffa41da91f6099f84f3443477edac448919a6d370c33b94d49ae17245df50f 243.29MB / 243.29MB                                                                                                       52.8s
 => => extracting sha256:993525952a483617183655b735562fd3eee4b3a85a472e3dc3c79a258adca100                                                                                                                 0.3s
 => => extracting sha256:12916f73f9de8e251c4eb4f2d079fa6cd63205b7ba8a7bd88e1f9446105e9ef1                                                                                                                 1.0s
 => => extracting sha256:371506b77bb850df2c7768a7e0e38b6259d82a0e6cd2d287800c151aa7d771c4                                                                                                                 3.0s
 => => extracting sha256:39ffa41da91f6099f84f3443477edac448919a6d370c33b94d49ae17245df50f                                                                                                                 3.7s
 => [internal] load build context                                                                                                                                                                         0.0s
 => => transferring context: 389B                                                                                                                                                                         0.0s
 => CACHED [stage-1 1/3] FROM docker.io/library/debian:buster-slim@sha256:eee33fc4994a42efcdd2fde591a82e2de62fa1a08c87c2508ef60cb4af438776                                                                0.0s
 => [builder 2/4] COPY . /app                                                                                                                                                                             0.2s
 => [builder 3/4] WORKDIR /app                                                                                                                                                                            0.0s
 => [builder 4/4] RUN cargo build --release                                                                                                                                                              61.2s
 => [stage-1 2/3] COPY --from=builder /app/target/release/rust-web-docker /app/rust-web-docker                                                                                                         0.0s
 => [stage-1 3/3] WORKDIR /app                                                                                                                                                                            0.0s
 => exporting to image                                                                                                                                                                                    0.0s
 => => exporting layers                                                                                                                                                                                   0.0s
 => => writing image sha256:5450e22d44781e515340f6d0230568ac521199a1c6b6830ab49a479636af7c7b                                                                                                              0.0s
 => => naming to docker.io/engelin/rust-web-docker
```

```bash
$ docker images
REPOSITORY                                      TAG       IMAGE ID       CREATED         SIZE
engelin/rust-web-docker                  latest    5450e22d4478   4 minutes ago   75.1MB
mariadb                                         10        3ac2017cb581   3 weeks ago     387MB
```

1.2 Run the Docker container
```bash
$ docker run --init -p 8080:8080 -it --rm --name rust-web engelin/rust-web-docker
```

1.3 Test the API
```bash
$ url -iX GET "http://localhost:8080?hello=test"
```

c.f. Curl test failure
```bash
$ curl http://localhost:8080
curl: (52) Empty reply from server
```
- Solution:
  - Check if the application inside a container should be binding to `0.0.0.0`

### 2. Development
2.1 Auto-reload
```bash
$ cargo watch -x run
```

### 3. Deployment to GCP
3.1 Install GCP CLI  
- Install GCP CLI  
```bash
$ brew install --cask google-cloud-sdk
$ source "$(brew --prefix)/share/google-cloud-sdk/path.zsh.inc"
$ source "$(brew --prefix)/share/google-cloud-sdk/completion.zsh.inc"
$ gcloud init
```

or  

```bash
$ curl https://sdk.cloud.google.com | bash
$ exec -l $SHELL
$ gcloud init
```

- Version check  
```bash
$ gcloud --version
Google Cloud SDK 445.0.0
bq 2.0.97
core 2023.09.01
gcloud-crc32c 1.0.0
gsutil 5.25
```

- Initialize the GCP CLI  
This command will create a new configuration or reinitialize an existing one, including login and setting up the default project.  
```bash
$ gcloud init
...
Network diagnostic passed (1/1 checks passed).

You must log in to continue. Would you like to log in (Y/n)?  y

...

You are logged in as: [yerin@arum3d.com].

Pick cloud project to use: 
 [1] xxxxx-xxxxx-398708
 [2] Enter a project ID
 [3] Create a new project
Please enter numeric choice or text value (must exactly match list item):  1

...

This gcloud configuration is called [default]. You can create additional configurations if you work with multiple accounts and/or projects.
Run `gcloud topic configurations` to learn more.

Some things to try next:

* Run `gcloud --help` to see the Cloud Platform services you can interact with. And run `gcloud help COMMAND` to get help on any gcloud command.
* Run `gcloud topic --help` to learn about advanced features of the SDK like arg files and output formatting
* Run `gcloud cheat-sheet` to see a roster of go-to `gcloud` commands.
```

3.2 Enable Cloud Build  
1) Go to Settings page (https://console.cloud.google.com/cloud-build/settings/()  
2) Click `View API`  
3) Click `Enable API`  
4) Go to Settings page again  

3.3 Upload a Docker image to Container Registry    
- Copy Google cloud ignore file  
```bash
$ cp -a .dockerignore .gcloudignore
```
And **remove Dockerfile** to build the Docker image on GCP.  

- Upload and build the Docker image on Google Cloud Build  
```bash
$ gcloud builds submit --tag gcr.io/${project-id}/${docker-image-name} --timeout=200
```

- Check the uploaded Docker image  
1) Go to Container Registry page (https://console.cloud.google.com/gcr/images/${project-id})

or  

2) Run the following command  
```bash
$ gcloud container images list
```

c.f. Use caching layer
```bash
$ gcloud config set builds/use_kaniko True
$ gcloud config list
[builds]
use_kaniko = True
[core]
account = xx@xxxd.com
disable_usage_reporting = False
project = xxxx-xxxx-xxxx

Your active configuration is: [default]
```

3.4 Deploy the Docker image to Cloud Run  
**Create service with web UI linked Github repo**  
1) Go to Cloud Run page (https://console.cloud.google.com/run)  
2) Click `Create Service`  
3) **Click `Continuously deploy new revisions from a source repository`**  
4) **Authorize Cloud Run to access your GitHub repository**  
5) **Select your GitHub repository**  
6) Click `Next`  
7) Select the Dockerfile to build the Docker image


- Create service with CLI  
```bash
$ gcloud run deploy --image gcr.io/${project-id}/${docker-image-name} --platform managed --region ${your-region} --allow-unauthenticated ${service-name}
```

### 4. Deployment to Heroku

4.1 Install Heroku CLI  
```bash
$ brew tap heroku/brew && brew install heroku
```

4.2 Login to Heroku  
```bash
$ heroku login
```

4.3 Test a Heroku app via CLI  
1) Create a Heroku app  
```bash
$ heroku create
```

2) Deploy the Docker image to Heroku  
```bash
$ heroku container:push web
$ heroku container:release web
```

3) Check the deployed Docker image  
```bash
$ heroku open
```

4) Check the deployed Docker image  
```bash
$ heroku logs --tail
```

5) Remove the Heroku app  
```bash
$ heroku apps:destroy
```

4.4 Deploy the Docker image to Heroku via Github Actions
1) Create a Heroku app  
```bash
$ heroku create
```

2) Create auth token  
```bash
$ heroku authorizations:create
Client:      <none>
ID:          ea17903e-xxxx-xxxx-xxxx-a21ed4c504a3
Description: Long-lived user authorization
Scope:       global
Token:       bd60xxxx-xxxx-xxxx-xxxx-xxxxx24401d6
Updated at:  Tue Sep 12 2023 14:02:46 GMT+0100 (British Summer Time) (less than a minute ago)
```

3) Add the following secrets to Github repo  
  - HEROKU_API_KEY : auth token  
  - HEROKU_APP_NAME : Heroku app name  

4.5 Check the deployed Docker image  
```bash
$ heroku open --app ${heroku-app-name}
```

4.6 Check the deployed Docker image  
```bash
$ heroku logs --tail --app ${heroku-app-name}
```

4.7 Remove the Heroku app  
```bash
$ heroku apps:destroy ${heroku-app-name}
 ▸    WARNING: This will delete ⬢ ${heroku-app-name} including all add-ons.
 ▸    To proceed, type r${heroku-app-name} or re-run this command with --confirm
 ▸    ${heroku-app-name}

> ${heroku-app-name}
Destroying ⬢ ${heroku-app-name} (including all add-ons)... done
```


### 5. Todos
- [ ] GCP Cloud Build optimization
  - Building via github actions is faster than building via GCP Cloud Build  
  - But both of them seems not to use caching layer

### 6. References
- [Google Cloud run tutorial for rust lang](https://www.youtube.com/watch?v=LRfraoVZDDg)
- [How to deploy a Rust Axum Web Server to Heroku using Docker and Github Actions](https://www.youtube.com/watch?v=ZF1WQGur_NA)
