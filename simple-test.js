const { getMdmSettings } = require('./index')

console.log('Testing MDM Config Reader...\n')

try {
  const settings = getMdmSettings()
  
  console.log('MDM Settings retrieved successfully!')
  console.log('Type:', typeof settings)
  console.log('Number of settings:', Object.keys(settings).length)
  
  if (Object.keys(settings).length > 0) {
    console.log('\nSettings:')
    for (const [key, value] of Object.entries(settings)) {
      console.log(`  ${key}: ${value}`)
    }
  } else {
    console.log('\nNo MDM settings found (empty object returned)')
    console.log('This is expected on non-Windows platforms or when no MDM configuration is present.')
  }
  
  console.log('\n✓ Simple test passed')
} catch (error) {
  console.error('✗ Error retrieving MDM settings:', error.message)
  process.exit(1)
}
