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

import pathlib

from fastapi import FastAPI
from google.adk.cli.fast_api import get_fast_api_app

# Set web=True if you intend to serve a web interface, False otherwise
SERVE_WEB_INTERFACE = False

# Call the function to get the FastAPI app instance
# Ensure the agent directory name ('zoo_adk_agent') matches your agent folder
app: FastAPI = get_fast_api_app(
    agents_dir=str(pathlib.Path(__file__).parent.resolve()),
    web=SERVE_WEB_INTERFACE,
)