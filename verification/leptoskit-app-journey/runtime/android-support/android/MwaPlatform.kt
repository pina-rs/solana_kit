package dev.solanakit.wallet

import android.content.ActivityNotFoundException
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import dev.leptoskit.bridge.NativePlugin
import dev.leptoskit.bridge.PluginException
import dev.leptoskit.bridge.utf8
import java.lang.ref.WeakReference
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.security.SecureRandom

/** Android foreground handover; cryptography and sockets remain in Rust. */
class MwaPlatform(context: Context) : NativePlugin {
    override val name = "solana-wallet-platform"
    override val methods: IntArray
        get() = intArrayOf(AVAILABLE, LAUNCH, ENTROPY)

    private val activity = WeakReference(foregroundActivity(context))
    private val random = SecureRandom()
    private var closed = false

    override suspend fun invoke(method: Int, payload: ByteBuffer): ByteArray {
        requireWallet(!closed, PluginException.UNAVAILABLE)

        return when (method) {
            AVAILABLE -> available(payload)
            LAUNCH -> launch(payload)
            ENTROPY -> entropy(payload)
            else -> throw PluginException(PluginException.UNAVAILABLE)
        }
    }

    private fun available(payload: ByteBuffer): ByteArray {
        requireWallet(!payload.hasRemaining())
        val current = activity.get()

        if (current == null) return byteArrayOf(0)

        val intent = Intent(Intent.ACTION_VIEW, Uri.parse("solana-wallet:/"))
        val installed = current.packageManager.resolveActivity(
            intent, PackageManager.MATCH_DEFAULT_ONLY,
        ) != null
        val usable = !current.isFinishing && !current.isDestroyed && installed

        return byteArrayOf(if (usable) 1 else 0)
    }

    // The foreground Activity owns handover. Original launch failures remain causes.
    private fun launch(payload: ByteBuffer): ByteArray {
        requireWallet(payload.remaining() in 1..MAX_URI_BYTES)
        val uri = Uri.parse(payload.utf8())
        validateAssociation(uri)
        val current = activity.get() ?: throw PluginException(PluginException.UNAVAILABLE)
        requireWallet(!current.isFinishing && !current.isDestroyed, PluginException.UNAVAILABLE)

        try {
            current.startActivity(Intent(Intent.ACTION_VIEW, uri))
        } catch (error: ActivityNotFoundException) {
            throw PluginException(PluginException.UNAVAILABLE, error)
        } catch (error: SecurityException) {
            throw PluginException(PluginException.PERMISSION_DENIED, error)
        }

        return byteArrayOf()
    }

    // Native provider failures are terminal; never fall back to seeded randomness.
    private fun entropy(payload: ByteBuffer): ByteArray {
        requireWallet(payload.remaining() == Int.SIZE_BYTES)
        val count = payload.asReadOnlyBuffer().order(ByteOrder.BIG_ENDIAN).int
        requireWallet(count in 1..MAX_ENTROPY_BYTES)
        val output = ByteArray(count)

        try {
            random.nextBytes(output)
        } catch (error: RuntimeException) {
            output.fill(0)
            throw PluginException(PluginException.NATIVE, error)
        }

        return output
    }

    /** No task is detached. A launched external Activity cannot be undone. */
    override fun close() {
        closed = true
        activity.clear()
    }

    companion object {
        const val AVAILABLE = 0
        const val LAUNCH = 1
        const val ENTROPY = 2
        const val MAX_ENTROPY_BYTES = 4096
        private const val MAX_URI_BYTES = 4096
    }
}
