language: rust 
rust:
     - stable
     - nightly
sudo: false
script: 
     - cargo build
     - cargo test
     - cargo run
deploy:
skip_cleanup: true
after_success:
  - eval "$(ssh-agent -s)" #start the ssh agent
  - chmod 600 .travis/deploy_key.pem # this key should have push access
  - ssh-add .travis/deploy_key.pem
  - git remote add deploy DEPLOY_REPO_URI_GOES_HERE
  - git push deploy