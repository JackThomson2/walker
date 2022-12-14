const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      return readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'server.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.android-arm64.node')
          } else {
            nativeBinding = require('@walkerserver/server-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'server.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.android-arm-eabi.node')
          } else {
            nativeBinding = require('@walkerserver/server-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'server.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@walkerserver/server-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'server.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@walkerserver/server-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'server.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@walkerserver/server-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'server.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.darwin-x64.node')
          } else {
            nativeBinding = require('@walkerserver/server-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'server.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.darwin-arm64.node')
          } else {
            nativeBinding = require('@walkerserver/server-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'server.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./server.freebsd-x64.node')
      } else {
        nativeBinding = require('@walkerserver/server-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'server.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./server.linux-x64-musl.node')
            } else {
              nativeBinding = require('@walkerserver/server-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'server.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./server.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@walkerserver/server-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'server.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./server.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@walkerserver/server-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'server.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./server.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@walkerserver/server-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'server.linux-arm-gnueabihf.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./server.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('@walkerserver/server-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { DbConnection, connectDb, PreparedStatement, Methods, newRoute, get, post, put, patch, RequestBlob, start, startWithWorkerCount, startWithConfig, stop, loadNewTemplate, reloadGroup, getThreadAffinity } = nativeBinding

module.exports.DbConnection = DbConnection
module.exports.connectDb = connectDb
module.exports.PreparedStatement = PreparedStatement
module.exports.Methods = Methods
module.exports.newRoute = newRoute
module.exports.get = get
module.exports.post = post
module.exports.put = put
module.exports.patch = patch
module.exports.RequestBlob = RequestBlob
module.exports.start = start
module.exports.startWithWorkerCount = startWithWorkerCount
module.exports.startWithConfig = startWithConfig
module.exports.stop = stop
module.exports.loadNewTemplate = loadNewTemplate
module.exports.reloadGroup = reloadGroup
module.exports.getThreadAffinity = getThreadAffinity
