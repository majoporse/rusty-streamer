"use client";

import { NewWatchHistory, WatchHistoryApi, WrapperMovie } from "@/generated";
import { AuthContainer } from "@/hooks/useAuth";
import { AxiosConfig } from "@/lib/utils";
import { FC, useEffect, useRef } from "react";
import videojs from "video.js";
import Player from "video.js/dist/types/player";
import "video.js/dist/video-js.css";

interface VideoPlayerProps extends React.HTMLAttributes<HTMLDivElement> {
  movie: WrapperMovie;
  options?: Player["options"];
}

export const VideoPlayer: FC<VideoPlayerProps> = ({
  movie,
  options,
  ...props
}) => {
  const auth = AuthContainer.useContainer();
  const videoRef = useRef<HTMLDivElement>(null);
  const playerRef = useRef<Player | null>(null);

  // Initialize player ONCE
  useEffect(() => {
    if (!playerRef.current && videoRef.current) {
      const videoElement = document.createElement("video");
      videoElement.classList.add(
        "video-js",
        "vjs-16-9",
        "rounded-2xl",
        "overflow-hidden"
      );

      videoRef.current.appendChild(videoElement);

      playerRef.current = videojs(videoElement, {
        controls: true,
        fluid: true,
        reactive: true,
        preload: "auto",
        poster: movie.poster_url,
        ...options,
      });

      const THROTTLE_MS = 5000;
      let lastSent = 0;

      console.log("Setting up video event listeners");
      playerRef.current.on("timeupdate", async () => {
        console.log("Time update:", playerRef.current?.currentTime());
        const now = Date.now();
        if (now - lastSent > THROTTLE_MS) {
          await sendProgress(playerRef.current?.currentTime());
          lastSent = now;
        }
      });

      playerRef.current.on("pause", async () => {
        console.log("Paused");
        await sendProgress(playerRef.current?.currentTime());
      });

      playerRef.current.on("seeked", async () => {
        console.log("Seeked");
        await sendProgress(playerRef.current?.currentTime());
      });

      playerRef.current.on("ended", async () => {});

      async function sendProgress(position: number | undefined) {
        let api = new WatchHistoryApi(AxiosConfig);
        let res = await api.createWatchHistory({
          completed: false,
          content_id: movie.id,
          progress_seconds: Math.floor(position ?? 0),
          user_id: auth.user?._id,
        } as NewWatchHistory);
      }

      // // Make control bar wrap when space is constrained to avoid overflow
      // const el = playerRef.current.el();
      // const controlBar = el.querySelector(
      //   ".vjs-control-bar"
      // ) as HTMLElement | null;
      // if (controlBar) {
      //   controlBar.style.flexWrap = "wrap";
      //   controlBar.style.gap = "0.5rem";
      //   controlBar.style.alignItems = "center";
      // }
    }
  }, [options, movie]);

  // Update video source when available
  useEffect(() => {
    if (!playerRef.current || !movie.video_url) return;

    playerRef.current.src({
      src: movie.video_url,
      type: "video/mp4",
    });
  }, [movie]);

  // Cleanup
  useEffect(() => {
    return () => {
      if (playerRef.current && !playerRef.current.isDisposed()) {
        playerRef.current.dispose();
        playerRef.current = null;
      }
    };
  }, []);

  return (
    <div
      data-vjs-player
      {...props}
      className={`w-full ${props.className ?? ""}`}
    >
      <div ref={videoRef} />
    </div>
  );
};
