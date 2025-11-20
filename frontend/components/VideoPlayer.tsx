"use client";

import { FC, useEffect, useRef } from "react";
import videojs from "video.js";
import Player from "video.js/dist/types/player";
import "video.js/dist/video-js.css";

interface VideoPlayerProps extends React.HTMLAttributes<HTMLDivElement> {
  src: string;
  posterSrc: string;
  options?: Player["options"];
}

export const VideoPlayer: FC<VideoPlayerProps> = ({
  src,
  posterSrc,
  options,
  ...props
}) => {
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
        "overflow-hidden",
      );

      videoRef.current.appendChild(videoElement);

      playerRef.current = videojs(videoElement, {
        controls: true,
        fluid: true,
        reactive: true,
        preload: "auto",
        poster: posterSrc,
        ...options,
      });

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
  }, [options, posterSrc]);

  // Update video source when available
  useEffect(() => {
    if (!playerRef.current || !src) return;

    playerRef.current.src({
      src,
      type: "video/mp4",
    });
  }, [src]);

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
