# Copyright 2025 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import os
import google.auth.transport.requests
import google.oauth2.id_token
from google.adk.agents import Agent
from google.adk.tools.mcp_tool import MCPToolset, StreamableHTTPConnectionParams

# Fetch the MCP server URL from an environment variable
MCP_SERVER_URL = os.environ.get("MCP_SERVER_URL")
if not MCP_SERVER_URL:
    raise ValueError("MCP_SERVER_URL environment variable not set.")

def get_auth_headers(mcp_server_url: str) -> dict[str, str]:
    """
    Fetches a Google-signed OIDC identity token and formats it as auth headers.
    The token is valid for invoking a private Cloud Run service.
    """
    # The audience for the token must be the root URL of the Cloud Run service.
    # We remove the '/mcp' path to get the base URL.
    audience = mcp_server_url.removesuffix("/mcp")

    auth_req = google.auth.transport.requests.Request()
    # This will fetch a cached token or a new one if it's expired.
    id_token = google.oauth2.id_token.fetch_id_token(auth_req, audience)

    return {"Authorization": f"Bearer {id_token}"}

# For a serverless environment like Cloud Run, fetching the token at startup
# is generally sufficient, as instances are short-lived. The ADK's
# MCPToolset takes static headers, so we provide them here.
mcp_tools = MCPToolset(
    connection_params=StreamableHTTPConnectionParams(
        url=MCP_SERVER_URL,
        headers=get_auth_headers(MCP_SERVER_URL)
    )
)

root_agent = Agent(
    model="gemini-2.5-flash",
    name="zoo_tour_guide",
    instruction="You are a helpful tour guide for a zoo. Use your tools to answer questions about the animals.",
    tools=[mcp_tools]
)
