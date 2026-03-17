import * as child_process from 'child_process';

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
type LightpandaServeOptions = {
    host?: string;
    port?: number;
    timeout?: number;
    disableHostVerification?: boolean;
    obeyRobots?: boolean;
    httpProxy?: string;
};

/**
 * @typedef LightpandaFetchOptions
 * @type {object}
 * @property {boolean} dump - Export fetched output as string
 * @property {boolean} disableHostVerification - Disables host verification on all HTTP requests
 * @property {boolean} obeyRobots - Fetches and obeys the robots.txt (if available) of the web pages we make requests towards.
 * @property {string} httpProxy - The HTTP proxy to use for all HTTP requests
 */
type LightpandaFetchOptions = {
    dump?: boolean;
    disableHostVerification?: boolean;
    obeyRobots?: boolean;
    httpProxy?: string;
};

declare const lightpanda: {
    fetch: (url: string, options?: LightpandaFetchOptions) => Promise<string | Buffer<ArrayBufferLike>>;
    serve: (options?: LightpandaServeOptions) => Promise<child_process.ChildProcessWithoutNullStreams>;
};

export { type LightpandaFetchOptions, type LightpandaServeOptions, lightpanda };
