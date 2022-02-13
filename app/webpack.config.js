import {fileURLToPath} from 'node:url'
import CompressionPlugin from 'compression-webpack-plugin'
import CssMinimzerWebpackPlugin from 'css-minimizer-webpack-plugin'
import CopyPlugin from 'copy-webpack-plugin'
import {constants as zlibConstants} from 'node:zlib'
import webpack from 'webpack'
import MiniCssExtractPlugin from 'mini-css-extract-plugin'
import sass from 'sass'
import {InjectManifest} from 'workbox-webpack-plugin'
import {
  resolveAssetsPath,
  resolveSrcPath,
  resolveOutputPath,
  dirname,
  join,
  relative,
  sep,
} from './webpack/paths.mjs'
import {buildWasm} from './webpack/build-wasm.mjs'
import {HtmlTemplateParameters} from './webpack/HtmlTemplateParameters.mjs'
import HtmlWebpackPlugin from 'html-webpack-plugin'

const {DefinePlugin} = webpack
const __dirname = dirname(fileURLToPath(import.meta.url))
const {BROTLI_PARAM_MODE, BROTLI_MODE_TEXT} = zlibConstants
const COMPRESSION_OPTIONS = {
  test: /\.((?:m?jsx?)|(?:(sa|sc|c)ss)|html)$/,
  threshold: 860,
}

/**
 *
 * @param {object} env
 * @param {object} argv
 * @returns {webpack.Configuration}
 */
export default async function configure(env, argv) {
  let {mode} = argv
  let {APP_VERSION} = env ?? {}
  APP_VERSION = APP_VERSION || '1'
  mode = mode || 'production'
  await buildWasm()
  return {
    devtool: mode !== 'production' ? 'inline-source-map' : undefined,
    mode,
    experiments: {
      topLevelAwait: true,
      outputModule: true,
      asyncWebAssembly: true,
    },
    entry: {
      main: resolveSrcPath('main.js'),
    },
    module: {
      rules: [
        {
          test: /\.m?js$/i,
          use: [
            {
              loader: 'babel-loader',
              options: {
                rootMode: 'upward',
                sourceMap: true,
              },
            },
          ],
        },
        {
          test: /\.(sc|sa|c)ss$/i,
          use: [
            MiniCssExtractPlugin.loader,
            'css-loader',
            'resolve-url-loader',
            {
              loader: 'postcss-loader',
              options: {sourceMap: true},
            },
            {
              loader: 'sass-loader',
              options: {
                sourceMap: true,
                implementation: sass,
              },
            },
          ],
        },
      ],
    },
    optimization: {
      flagIncludedChunks: true,
      providedExports: true,
      sideEffects: true,
      usedExports: true,
      removeEmptyChunks: true,
      concatenateModules: false,
      runtimeChunk: 'single',
      moduleIds: 'deterministic',
      chunkIds: 'deterministic',
      minimize: true,
      minimizer: [
        '...',
        new CssMinimzerWebpackPlugin({
          minimizerOptions: {
            preset: 'cssnano-preset-advanced',
          },
        }),
      ],
      splitChunks: {
        hidePathInfo: true,
        minSize: 860,
        name: false,
        maxAsyncRequests: Infinity,
        maxInitialRequests: 6,
        automaticNameDelimiter: sep,
        cacheGroups: {
          lib: {
            chunks: 'all',
            name(module, chunks, cacheGroupKey) {
              const id = module.identifier()
              const packageNameParts = id
                .replace(/.+[\\/]node_modules[\\/]@?/i, '')
                .replace(/[\\/](build|dist)/, '')
                .replace(/(\.(es|((c|m)?js)))+$/i, '')
                .split(/[\\/]/)
              const fileNameIndex = packageNameParts.length - 1
              if (packageNameParts[fileNameIndex]?.toLowerCase() === 'index') {
                packageNameParts[fileNameIndex] =
                  packageNameParts[fileNameIndex - 1]
              }

              return join(cacheGroupKey, ...packageNameParts)
            },
            test({type, context}) {
              return (
                /^javascript/.test(type) &&
                /[\\/]node_modules[\\/]/.test(context)
              )
            },
            reuseExistingChunk: true,
            enforce: true,
          },
          defaultVendors: false,
          default: false,
        },
      },
    },
    output: {
      filename: '[name].[chunkhash].js',
      path: resolveOutputPath(),
      clean: true,
      module: true,
    },
    plugins: [
      new DefinePlugin({
        __APP_VERSION__: JSON.stringify(APP_VERSION),
      }),
      new CopyPlugin({
        patterns: [
          {
            from: resolveAssetsPath('**', '*.{ico,png}'),
            to({absoluteFilename}) {
              return relative(resolveAssetsPath(), absoluteFilename)
            },
          },
        ],
      }),
      new MiniCssExtractPlugin({
        experimentalUseImportModule: false,
        filename: join('css', '[name].[chunkhash].css'),
      }),
      new CompressionPlugin(COMPRESSION_OPTIONS),
      new CompressionPlugin({
        ...COMPRESSION_OPTIONS,
        algorithm: 'brotliCompress',
        filename: '[path][base].br[query]',
        compressionOptions: {
          level: 11,
          params: {
            [BROTLI_PARAM_MODE]: BROTLI_MODE_TEXT,
          },
        },
      }),
      new HtmlWebpackPlugin({
        template: resolveAssetsPath('index.ejs'),
        favicon: resolveAssetsPath('favicon.ico'),
        inject: false,
        templateParameters(compilation, assets) {
          const parameters = new HtmlTemplateParameters()
          if (mode !== 'production') {
            parameters.addScript('/_framework/aspnetcore-browser-refresh.js')
          }
          for (const stylesheet of assets.css) {
            parameters.addStyleSheet(stylesheet)
          }
          for (const script of assets.js) {
            const {javascriptModule = false} =
              compilation.assetsInfo.get(script)
            if (script.startsWith('runtime')) continue
            parameters.addScript(script, javascriptModule)
          }
          return parameters
        },
      }),
      new InjectManifest({
        swSrc: resolveSrcPath('sw.js'),
        exclude: [/\.LICENSE.txt$/i, /\.map\..+$/i, /assets-manifest\.json/i],
      }),
    ],
  }
}
