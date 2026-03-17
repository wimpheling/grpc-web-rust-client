/**
 * Copyright 2023-2025 Lightpanda (Selecy SAS)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
import { type ChildProcessWithoutNullStreams } from 'node:child_process';
/**
 * @typedef LightpandaServeOptions
 * @type {object}
 * @property {string} host - Host of the CDP server
 * @property {string} port - Port of the CDP server
 * @property {number} timeout - Inactivity timeout in seconds before disconnecting clients
 * @property {boolean} disableHostVerification - Disables host verification on all HTTP requests
 * @property {boolean} obeyRobots - Fetches and obeys the robots.txt (if available) of the web pages we make requests towards.
 * @property {string} httpProxy - The HTTP proxy to use for all HTTP requests
 */
export type LightpandaServeOptions = {
    host?: string;
    port?: number;
    timeout?: number;
    disableHostVerification?: boolean;
    obeyRobots?: boolean;
    httpProxy?: string;
};
/**
 * Start a websocket CDP server
 * @param {LightpandaServeOptions} options - Options to pass to Lightpanda
 * @returns {Promise<ChildProcessWithoutNullStreams>}
 */
export declare const serve: (options?: LightpandaServeOptions) => Promise<ChildProcessWithoutNullStreams>;
