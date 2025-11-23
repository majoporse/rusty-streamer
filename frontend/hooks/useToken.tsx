"use client";
import Axios, { AxiosRequestConfig, InternalAxiosRequestConfig } from 'axios';

import { useCallback, useEffect, useRef } from 'react';
import { configure } from 'axios-hooks';
import { useTokenExpiration } from './useTokenExpiration';
import { User } from './useAuth';
import axios from 'axios';
import { AxiosConfig } from '@/lib/utils';
import { AuthApi } from '@/generated';

export interface TokenResponse {
  access_token: string;
  token_expiration: number;
}

export interface UserAndTokenResponse extends TokenResponse {
  user: User;
}

export function useToken(onTokenInvalid: Function, onRefreshRequired: Function) {
  const accessToken = useRef<string>(null);
  const { clearAutomaticTokenRefresh, setTokenExpiration } = useTokenExpiration(onRefreshRequired);

  const setToken = useCallback(
    ({ token_expiration, access_token }: TokenResponse) => {
      accessToken.current = access_token;
      const expirationDate = new Date(token_expiration);
      setTokenExpiration(expirationDate);
    },
    [setTokenExpiration],
  );

  const isAuthenticated = useCallback(() => {
    return !!accessToken.current;
  }, []);

  const clearToken = useCallback(
    (shouldClearCookie = true) => {
      // if we came from a different tab, we should not clear the cookie again
      let api = new AuthApi(AxiosConfig);
      const clearRefreshTokenCookie = shouldClearCookie ? api.logout() : Promise.resolve();

      // clear refresh token
      return clearRefreshTokenCookie.finally(() => {
        // clear token
        accessToken.current = '';

        // clear auto refresh interval
        clearAutomaticTokenRefresh();
      });
    },
    [clearAutomaticTokenRefresh],
  );

  useEffect(() => {
    // add authorization token to each request
    axios.interceptors.request.use(
      (config: InternalAxiosRequestConfig): InternalAxiosRequestConfig => {
        config.headers.authorization = `Bearer ${accessToken.current}`;
        return config;
      },
    );

    // if the current token is expired or invalid, logout the user
    axios.interceptors.response.use(
      (response) => response,
      (error) => {
        if (error.response.status === 401 && accessToken.current) {
          clearToken();

          // let the app know that the current token was cleared
          onTokenInvalid();
        }
        console.log("Response error intercepted:", error.response);
        return Promise.reject(error);
      },
    );

    // configure axios-hooks to use this instance of axios
    configure({ axios });
  }, [clearToken, onTokenInvalid]);

  return {
    clearToken,
    setToken,
    isAuthenticated,
  };
}
