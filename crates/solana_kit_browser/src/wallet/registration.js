/** Adapts the Wallet Standard variadic registration API to one typed array. */
export function registration(callback) {
  return (...wallets) => callback(wallets);
}
