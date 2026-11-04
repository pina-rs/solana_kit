package dev.solanakit.wallet

import android.app.Activity
import android.content.Context
import android.content.ContextWrapper

private const val MAX_CONTEXT_LAYERS = 16

/** A plugin never promotes an application Context into an Activity capability. */
internal fun foregroundActivity(context: Context): Activity? {
    var current = context

    for (layer in 0 until MAX_CONTEXT_LAYERS) {
        val value = current

        if (value is Activity) return value

        val next = (value as? ContextWrapper)?.baseContext ?: return null

        if (next === current) return null

        current = next
    }

    return null
}
