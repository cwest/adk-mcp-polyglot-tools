# ADK, MCP, and Cloud Run: A Polyglot Playbook

This repository contains the source code for the "Building Scalable AI Agents"
playbook. It demonstrates how to build a decoupled, microservices-based AI agent
system using the Agent Development Kit (ADK), the Model Context Protocol (MCP),
and Google Cloud Run.

The project consists of two main components:

1.  **`zoo_mcp_server_rust`**: A secure, private tool server built in Rust that
    exposes a simple API for getting information about zoo animals.
2.  **`zoo_adk_agent`**: A public-facing AI agent built with the Python ADK. It
    uses the Rust MCP server as a tool to answer user questions.

## Learn More

For a deeper dive into the concepts and benefits of building decoupled AI agents
with the ADK and MCP, check out the companion blog post:

- [**Building Scalable AI Agents: A Deep Dive into Decoupled Tools with ADK, MCP, and Cloud Run**](https://caseywest.com/building-scalable-ai-agents-a-deep-dive-into-decoupled-tools-with-adk-mcp-and-cloud-run/)

This article explores the production-grade advantages of implementing agentic
Tools in separate, secure MCP servers.

## Build Your Own

Ready to build your own? The best way to solidify these concepts is to get

> hands-on. Follow this step-by-step tutorial to
> [deploy your own secure, remote MCP server on Google Cloud](https://cloud.google.com/run/docs/tutorials/deploy-remote-mcp-server?utm_campaign=CDR_0x5d16fa53_default&utm_medium=external&utm_source=blog)
> and start building a truly scalable tool ecosystem for your agents.

## Deploying to Google Cloud

These instructions will guide you through deploying both services to Cloud Run.

### 1. Prerequisites

- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install) (`gcloud`)
  installed and authenticated.
- [Docker](https://docs.docker.com/get-docker/) installed and running.
- A Google Cloud project with billing enabled.

### 2. Set Up Environment Variables

First, configure your environment with your project ID and preferred region.

```sh
export PROJECT_ID=$(gcloud config get-value project)
export REGION="us-central1" # Or your preferred region
```

### 3. Enable Google Cloud APIs

Enable the necessary APIs for Cloud Run, Artifact Registry, Cloud Build, and
IAM.

```sh
gcloud services enable \
    run.googleapis.com \
    aiplatform.googleapis.com \
    iam.googleapis.com \
    artifactregistry.googleapis.com \
    cloudbuild.googleapis.com
```

### 4. Deploy the Rust MCP Server

This command builds the Rust server's container image and deploys it as a
**private** Cloud Run service.

```sh
gcloud run deploy zoo-mcp-server-rust \
  --source ./zoo_mcp_server_rust \
  --region=$REGION \
  --no-allow-unauthenticated
```

### 5. Grant Permissions

The ADK agent needs permission to invoke the private MCP server. This command
grants the agent's default service account the "Cloud Run Invoker" role for the
`zoo-mcp-server-rust` service.

```sh
# Get the default service account for the ADK agent
PROJECT_NUMBER=$(gcloud projects describe $PROJECT_ID --format="value(projectNumber)")
AGENT_SA="${PROJECT_NUMBER}-compute@developer.gserviceaccount.com"

# Grant the invoker role
gcloud run services add-iam-policy-binding zoo-mcp-server-rust \
  --member="serviceAccount:${AGENT_SA}" \
  --role="roles/run.invoker" \
  --region=$REGION
```

### 6. Deploy the Python ADK Agent

This command builds the Python agent's container and deploys it as a **public**
Cloud Run service. It injects the private MCP server's URL as an environment
variable.

```sh
# Get the URL of the deployed MCP server
MCP_URL=$(gcloud run services describe zoo-mcp-server-rust --region=$REGION --format="value(status.url)")

# Deploy the agent, passing the MCP server's URL
gcloud run deploy zoo-adk-agent \
  --source ./zoo_adk_agent \
  --region=$REGION \
  --allow-unauthenticated \
  --set-env-vars="GOOGLE_CLOUD_PROJECT=$PROJECT_ID,GOOGLE_CLOUD_LOCATION=$REGION,GOOGLE_GENAI_USE_VERTEXAI=TRUE,MCP_SERVER_URL=${MCP_URL}/mcp"
```

### 7. Test the System

Once both services are deployed, you can interact with the agent using `curl`.

```sh
# Get the agent's public URL
AGENT_URL=$(gcloud run services describe zoo-adk-agent --region=$REGION --format="value(status.url)")

# Initialize a session
curl -X POST $AGENT_URL/apps/zoo_tour_guide/users/user_123/sessions/session_abc \
    -H "Content-Type: application/json"

# Send a prompt to the agent
curl -X POST $AGENT_URL/run_sse \
    -H "Content-Type: application/json" \
    -d '{
    "app_name": "zoo_tour_guide",
    "user_id": "user_123",
    "session_id": "session_abc",
    "new_message": {
        "role": "user",
        "parts": [{
            "text": "Hello, tell me about the lions."
        }]
    },
    "streaming": false
    }'
```

You should receive a JSON response from the agent containing details about
lions, which it fetched securely from the Rust MCP server.

````

## Teardown

To avoid incurring future charges, delete the Cloud Run services.

```sh
gcloud run services delete zoo-mcp-server-rust --region=$REGION --quiet
gcloud run services delete zoo-adk-agent --region=$REGION --quiet
````
