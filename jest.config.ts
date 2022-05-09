import {Config} from '@jest/types'

export default {
  preset: 'ts-jest',
  clearMocks: true,
  coverageDirectory: 'coverage',
  testEnvironment: 'node',
  moduleFileExtensions: ['ts', 'js', 'json', 'tsx'],
  moduleDirectories: ['node_modules'],
  setupFiles: ['./tests/setup.ts'],
  globals: {
    'ts-jest': {
      tsconfig: 'tsconfig.test.json'
    }
  },
  testTimeout: 1000 * 60 * 10
} as Config.InitialOptions;
