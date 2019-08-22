#include <stdlib.h>

#include "weechat-plugin.h"

WEECHAT_PLUGIN_NAME("example");
WEECHAT_PLUGIN_DESCRIPTION("Example plugin for WeeChat");
WEECHAT_PLUGIN_AUTHOR("matematikaadit <matematikaadit@gmail.com>");
WEECHAT_PLUGIN_VERSION("0.1");
WEECHAT_PLUGIN_LICENSE("MIT");

struct t_weechat_plugin *weechat_plugin = NULL;


int
weechat_plugin_init (struct t_weechat_plugin *plugin,
                     int argc, char *argv[])
{
    weechat_plugin = plugin;
    return WEECHAT_RC_OK;
}

int
weechat_plugin_end (struct t_weechat_plugin *plugin)
{
    /* make C compiler happy */
    (void) plugin;
    return WEECHAT_RC_OK;
}
