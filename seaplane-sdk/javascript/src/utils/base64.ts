const base64 = require('urlsafe-base64'); // eslint-disable-line

export const encode = (data: string): string => base64.encode(Buffer.from(data));

export const decode = (data: string): string => base64.decode(data).toString();