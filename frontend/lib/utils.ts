import { Configuration } from "@/generated";
import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const AxiosConfig = new Configuration({
  basePath: "http://localhost:8085",
});