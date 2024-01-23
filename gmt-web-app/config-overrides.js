/* eslint-disable */

module.exports = function override(config, env) {
  // New config, e.g. config.plugins.push...
  config.resolve.alias = {
    ...config.resolve.alias,
    '@': require('path').resolve(__dirname, 'src'),
  }
  return config
}