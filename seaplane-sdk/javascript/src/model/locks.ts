import { decode } from '../utils/base64';

export type Name = {
  name: string;
};

export type LockInfo = {
  ttl: number;
  clientId: string;
  ip: string;
};

export type Lock = {
  id: string;
  name: Name;
  info: LockInfo;
};

export type LockPage = {
  locks: [Lock];
  nextLock?: Name | null;
};

export type HeldLock = {
  id: string;
  sequencer: number;
};

export const toLockInfo = (lockInfo: any): LockInfo => ({  // eslint-disable-line
  ttl: lockInfo['ttl'],
  clientId: lockInfo['client-id'],
  ip: lockInfo['ip'],
});

const toName = (name?: string): Name | null => {
  if (!name) return null;

  return {
    name: decode(name),
  };
};

export const toLock = (lock: any): Lock => { // eslint-disable-line
  const lockName = toName(lock['name']);
  let name = { name: '' };
  if (lockName) {
    name = lockName;
  }
  return {
    id: lock['id'],
    name,
    info: toLockInfo(lock['info']),
  };
};

export const toLockPage = (lockPage: any): LockPage => { // eslint-disable-line
  return {
    locks: lockPage['infos'].map((lock: any) => toLock(lock)), // eslint-disable-line
    nextLock: toName(lockPage['next']),
  };
};

export const toHeldLock = (lock: any): HeldLock => ({ // eslint-disable-line
  id: lock['id'],
  sequencer: lock['sequencer'],
});
