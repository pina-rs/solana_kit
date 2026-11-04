package dev.solanakit.wallet

import android.net.Uri
import dev.leptoskit.bridge.PluginException

private const val MIN_PORT = 49152
private const val MAX_PORT = 65535
private val ASSOCIATION_FIELDS = setOf("association", "port", "v")
private val PUBLIC_POINT = Regex("[A-Za-z0-9_-]{87}")

/** Validates only the bounded local-v1 URI emitted by the portable protocol. */
internal fun validateAssociation(uri: Uri) {
    validateScheme(uri)
    requireWallet(uri.fragment == null)
    requireWallet(uri.path.orEmpty().endsWith("/v1/associate/local"))
    requireWallet(uri.queryParameterNames == ASSOCIATION_FIELDS)

    for (field in ASSOCIATION_FIELDS) {
        requireWallet(uri.getQueryParameters(field).size == 1)
    }

    val port = uri.getQueryParameter("port")?.toIntOrNull()
        ?: throw PluginException(PluginException.INVALID_ARGUMENT)
    val key = uri.getQueryParameter("association")
        ?: throw PluginException(PluginException.INVALID_ARGUMENT)
    requireWallet(port in MIN_PORT..MAX_PORT)
    requireWallet(uri.getQueryParameter("v") == "v1")
    requireWallet(PUBLIC_POINT.matches(key))
}

private fun validateScheme(uri: Uri) {
    when (uri.scheme) {
        "solana-wallet" -> requireWallet(uri.authority.isNullOrEmpty())
        "https" -> {
            requireWallet(!uri.host.isNullOrEmpty())
            requireWallet(uri.encodedAuthority?.contains('@') == false)
        }
        else -> throw PluginException(PluginException.INVALID_ARGUMENT)
    }
}

/** Keeps every platform validation failure in the bridge's typed error domain. */
internal fun requireWallet(valid: Boolean, code: Int = PluginException.INVALID_ARGUMENT) {
    if (!valid) throw PluginException(code)
}
