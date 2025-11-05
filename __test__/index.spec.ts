import test from 'ava'

import { getMdmSettings } from '../index.js'

test('getMdmSettings returns an object', (t) => {
  const settings = getMdmSettings()
  t.is(typeof settings, 'object')
  t.false(Array.isArray(settings))
})

test('getMdmSettings values are all strings', (t) => {
  const settings = getMdmSettings()

  // All values in the returned object should be strings.
  for (const value of Object.values(settings)) {
    t.is(typeof value, 'string')
  }

  t.pass('All MDM settings values are strings')
})
