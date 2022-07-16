pipeline {
  agent {
    docker {
      image 'rust:latest'
    }

  }
  stages {
    stage('Build') {
      steps {
        sh 'cargo build'
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

    stage('Doc') {
      steps {
        sh 'cargo doc'
        step([$class: 'JavadocArchiver',
                                      javadocDir: 'target/doc',
                                      keepAll: false])
      }
    }

  }
  environment {
    DOCKER_HOST = 'tcp://docker:2376'
    DOCKER_CERT_PATH = '/certs/client'
    DOCKER_TLS_VERIFY = '1'
  }
}