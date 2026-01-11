import axios, { type AxiosRequestConfig } from "axios";

const getBaseURL = () => {
  const apiUrl = import.meta.env.VITE_API_URL;
  if (apiUrl && apiUrl.trim() !== "") {
    return apiUrl;
  }
  return "";
};

const axiosInstance = axios.create({
  baseURL: getBaseURL(),
});

let tokenGetter: (() => Promise<string | null>) | null = null;

export const setAuthTokenGetter = (getter: () => Promise<string | null>) => {
  tokenGetter = getter;
};

axiosInstance.interceptors.request.use(
  async (config) => {
    if (tokenGetter) {
      try {
        const token = await tokenGetter();
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
      } catch (error) {
        console.error("Failed to get auth token:", error);
      }
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

class APIClient<T> {
  endpoint: string;

  constructor(endpoint: string) {
    this.endpoint = endpoint;
  }

  getAll = (config?: AxiosRequestConfig) => {
    return axiosInstance
      .get<T[]>(this.endpoint, config)
      .then((res) => res.data);
  };

  get = (id: number | string, config?: AxiosRequestConfig) => {
    return axiosInstance
      .get<T>(this.endpoint + "/" + id, config)
      .then((res) => res.data);
  };

  post = <PostT, ResponseT = T>(data: PostT, path?: string) => {
    const url = path ? this.endpoint + "/" + path : this.endpoint;
    return axiosInstance.post<ResponseT>(url, data).then((res) => res.data);
  };

  patch = <PatchT>(id: number | string, data: PatchT) => {
    return axiosInstance
      .patch<T>(this.endpoint + "/" + id, data)
      .then((res) => res.data);
  };

  delete = (id: number | string) => {
    return axiosInstance
      .delete<T>(this.endpoint + "/" + id)
      .then((res) => res.data);
  };
}

export default APIClient;
export { axiosInstance };
