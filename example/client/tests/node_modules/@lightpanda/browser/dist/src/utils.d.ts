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
import { type BinaryToTextEncoding } from 'node:crypto';
export declare const DEFAULT_CACHE_FOLDER: string;
export declare const BINARY_NAME = "lightpanda";
export declare const USER_EXECUTABLE_PATH: string;
export declare const DEFAULT_EXECUTABLE_PATH: string;
export declare const GITHUB_RELEASE_DATA_URL = "https://api.github.com/repos/lightpanda-io/browser/releases/tags/nightly";
/**
 * Validate a URL structure
 * @param {string} url URL to validate
 */
export declare const validateUrl: (url: string) => void;
/**
 * Validate a port number
 * @param {number} port Port number to validate
 */
export declare const validatePort: (port: number) => void;
/**
 * Get executable path
 */
export declare const getExecutablePath: () => string;
/**
 * Get checksum from file
 */
export declare const checksumFile: (filePath: string, algorithm?: string, encoding?: BinaryToTextEncoding) => Promise<unknown>;
