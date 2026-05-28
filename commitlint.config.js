module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    // Lu bisa tambahin aturan custom di sini kalau mau
    'subject-case': [2, 'always', 'sentence-case']
  }
};
