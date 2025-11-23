import { Configuration } from "@/generated";
import axios from "axios";
import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const AxiosConfig = new Configuration({
  basePath: "http://localhost:8085",
  baseOptions: {
    withCredentials: true,     // 🔥 required to receive cookies
  }
});

axios.defaults.baseURL = AxiosConfig.basePath;
axios.defaults.withCredentials = true;