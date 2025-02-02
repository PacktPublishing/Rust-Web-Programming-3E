module.exports = {
    preset: 'ts-jest',
    testEnvironment: 'jest-environment-jsdom',
    setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
    transform: {
        '^.+\\.(ts|tsx)$': ['ts-jest', {
            tsconfig: 'ts.config.json'
            // any other ts-jest specific options go here
        }],
    },
    moduleNameMapper: {
        // Add if you have styles or assets imported in your components
        "\\.(css|less|scss|sass)$": "identity-obj-proxy"
    }
};
