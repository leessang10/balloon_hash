const { BalloonHash } = require('./lib');

/**
 * Balloon Hash implementation following RFC 9197
 * @param {Uint8Array|string} password - The password to hash
 * @param {Uint8Array|string} salt - The salt value
 * @param {Object} options - Hash options
 * @param {number} [options.spaceCost=16] - Memory usage (number of blocks)
 * @param {number} [options.timeCost=20] - Number of iterations
 * @param {number} [options.parallelCost=1] - Degree of parallelism
 * @returns {Uint8Array} The resulting hash
 */
function balloonHash(password, salt, options = {}) {
  const { spaceCost = 16, timeCost = 20, parallelCost = 1 } = options;

  // Convert string inputs to Uint8Array if necessary
  const passwordBuffer = typeof password === 'string' ? new TextEncoder().encode(password) : password;

  const saltBuffer = typeof salt === 'string' ? new TextEncoder().encode(salt) : salt;

  const balloon = new BalloonHash(spaceCost, timeCost, parallelCost);
  return balloon.hash(passwordBuffer, saltBuffer);
}

module.exports = {
  balloonHash,
  BalloonHash,
};
