import { describe, expect, test } from 'vitest';

import { TTS_CACHE_REQUIRES_PREMIUM, isTTSCacheAllowed, isTTSCacheInPlan } from '@/utils/access';

describe('isTTSCacheInPlan', () => {
  test('every plan includes the offline TTS audio cache', () => {
    expect(isTTSCacheInPlan('free')).toBe(true);
    expect(isTTSCacheInPlan('plus')).toBe(true);
    expect(isTTSCacheInPlan('pro')).toBe(true);
    expect(isTTSCacheInPlan('purchase')).toBe(true);
  });
});

describe('isTTSCacheAllowed (premium paywall removed)', () => {
  test('offline TTS audio is available to all plans', () => {
    expect(TTS_CACHE_REQUIRES_PREMIUM).toBe(false);
    expect(isTTSCacheAllowed('free')).toBe(true);
    expect(isTTSCacheAllowed('plus')).toBe(true);
    expect(isTTSCacheAllowed('pro')).toBe(true);
    expect(isTTSCacheAllowed('purchase')).toBe(true);
  });
});
