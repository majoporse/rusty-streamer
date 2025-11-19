import type videojs from "video.js";

declare module "video.js" {
    /**
     * Options accepted by the videojs-playlist-ui plugin.
     * Extends the Video.js ComponentOptions so common component props are available.
     */
    namespace videojsPlaylistUi {
        interface Options extends videojs.ComponentOptions {
            /** CSS class applied to the playlist container. Default: 'vjs-playlist' */
            className?: string;

            /** Whether selecting an item starts playback immediately. Default: false */
            playOnSelect?: boolean;
        }
    }

    // Augment the player instance to expose the plugin method
    interface VideoJsPlayer {
        /**
         * Initialize or configure the playlist UI plugin on this player.
         * Call signature mirrors many video.js plugins: `player.playlistUi(options)`.
         */
        playlistUi?: (options?: videojsPlaylistUi.Options) => void;
    }
}

export { };
