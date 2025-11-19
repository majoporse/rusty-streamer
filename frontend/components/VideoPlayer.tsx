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
      videoElement.classList.add("video-js", "vjs-big-play-centered");

      videoRef.current.appendChild(videoElement);

      playerRef.current = videojs(videoElement, {
        controls: true,
        fluid: true,
        preload: "auto",
        poster: posterSrc,
        ...options,
      });
    }
  }, []);

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
      className={`w-full rounded-2xl overflow-hidden shadow-md bg-muted ${
        props.className || ""
      }`}
    >
      <div ref={videoRef} />
    </div>
  );
};
