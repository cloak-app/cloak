import { readFileSync, writeFileSync, watchFile } from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import SemVer from 'semver';
import { confirm, select } from '@inquirer/prompts';

/**
 * 根据版本号，判断可以进行的 release 类型
 * @param {string} version
 * @param {import('semver').ReleaseType} releaseType
 * @returns {string | undefined}
 */
const getDisabledReleaseType = (version, releaseType) => {
  const semver = SemVer.parse(version);

  // 非预发版本，不能直接执行 release 操作
  if (!semver.prerelease.length && releaseType === 'release') {
    return '(cannot release a non-prerelease version)';
  }
};

/**
 * 根据版本号，判断可以选择的 identifier 类型
 * @param {string} version
 * @param {string} identifier
 * @returns {string | undefined}
 */
const getDisabledIdentifier = (version, identifier) => {
  const semver = SemVer.parse(version);

  // 如果上一个版本是预发布的 beta 版本，则不允许发布 alpha 版本
  if (
    identifier &&
    semver.prerelease.length &&
    !semver.prerelease.includes(identifier)
  ) {
    return `(cannot change identifier from ${semver.prerelease[0]} to ${identifier})`;
  }
};

/**
 * 用于在修改 cargo.toml 后监听 Cargo.lock 文件更新，以完成版本更新
 * @param {string} filePath
 * @returns {Promise<void>}
 */
const watchFileUpdated = (filePath) => {
  return new Promise((resolve, reject) => {
    // 10 秒超时
    let timer = setTimeout(() => {
      console.error('[ERROR]: `Cargo.lock` updated timeout');
      reject();
    }, 10 * 1000);

    watchFile(filePath, () => {
      clearTimeout(timer);
      resolve();
    });
  });
};

async function main() {
  /* -------------------------------- 读取原始文件内容 -------------------------------- */
  const packageJsonPath = path.join('package.json');
  const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));

  const tauriConfPath = path.join('src-tauri', 'tauri.conf.json');
  const tauriConf = JSON.parse(readFileSync(tauriConfPath, 'utf8'));

  const cargoTomlPath = path.join('src-tauri', 'Cargo.toml');
  const cargoToml = readFileSync(cargoTomlPath, 'utf8');

  const cargoLockFilePath = path.join('src-tauri', 'Cargo.lock');

  // 获取旧版本号
  const oldVersion = packageJson.version;

  /* ------------------------------ 获取 release 参数 ----------------------------- */
  const releaseType = await select({
    message: 'Select a release type',
    choices: [
      'major',
      'premajor',
      'minor',
      'preminor',
      'patch',
      'prepatch',
      'release',
      'prerelease',
    ].map((releaseType) => ({
      name: releaseType,
      value: releaseType,
      disabled: getDisabledReleaseType(oldVersion, releaseType),
    })),
  });

  let identifier;

  // 如果是预发版本，则需要选择预发布标识符（beta、alpha）
  if (releaseType.startsWith('pre')) {
    identifier = await select({
      message: 'Select a identifier',
      choices: ['beta', 'alpha'].map((identifier) => ({
        name: identifier,
        value: identifier,
        disabled: getDisabledIdentifier(oldVersion, identifier),
      })),
    });
  }

  /* --------------------------------- 计算新版本号 --------------------------------- */
  const newVersion = SemVer.inc(packageJson.version, releaseType, identifier);

  const answer = await confirm({
    message: `New version will be \`${newVersion}\`, continue?`,
  });

  if (!answer) {
    console.log('[INFO]: Aborted');
    process.exit(0);
  }

  /* --------------------------------- 更新文件内容 --------------------------------- */
  packageJson.version = newVersion;
  tauriConf.version = newVersion;
  const newCargoToml = cargoToml.replace(
    /version = "(.*)"/,
    `version = "${newVersion}"`,
  );

  writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));
  console.log('[INFO]: `package.json` updated');
  writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2));
  console.log('[INFO]: `tauri.conf.json` updated');
  writeFileSync(cargoTomlPath, newCargoToml);
  console.log('[INFO]: `Cargo.toml` updated');

  // 需要等待 Cargo.lock 文件中的版本也自动更新
  await watchFileUpdated(cargoLockFilePath);
  console.log('[INFO]: `Cargo.lock` updated');

  /* ------------------------------- commit 版本更新 ------------------------------ */
  try {
    execSync('git add .', { stdio: 'inherit' });
    execSync(`git commit -m "chore(release): v${newVersion}"`, {
      stdio: 'inherit',
    });
    execSync('git push origin main', { stdio: 'inherit' });
    console.log('[INFO]: Version changes committed and pushed to main branch.');
  } catch (e) {
    console.error('[ERROR]: Failed to commit version changes:', e);
    process.exit(1);
  }

  /* ---------------------------------- 打 tag --------------------------------- */
  try {
    execSync(`git tag v${newVersion}`, { stdio: 'inherit' });
    execSync(`git push origin v${newVersion}`, { stdio: 'inherit' });
    console.log(`[INFO]: Git tag v${newVersion} created and pushed.`);
  } catch (e) {
    console.error(`[ERROR]: Failed to create or push git tag: v${newVersion}`);
    process.exit(0);
  }

  process.exit(0);
}

main();
