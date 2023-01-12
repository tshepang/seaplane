export const headers = (auth: string) => ({
  Accept: 'application/json',
  'Content-Type': 'application/json',
  Authorization: `Bearer ${auth}`,
});

const seaFetch = (token: string) => ({
  get: async (url: string) =>
    fetch(url, {
      method: 'GET',
      headers: headers(token),
    }),
  post: async (url: string, body: string) =>
    await fetch(url, {
      method: 'POST',
      headers: headers(token),
      body,
    }),
  put: async (url: string, body: string) =>
    await fetch(url, {
      method: 'PUT',
      headers: headers(token),
      body,
    }),
  delete: async (url: string) =>
    fetch(url, {
      method: 'DELETE',
      headers: headers(token),
    }),
  patch: async (url: string) =>
    fetch(url, {
      method: 'PATCH',
      headers: headers(token),
    }),
});

export default seaFetch;
