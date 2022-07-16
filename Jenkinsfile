pipeline {
  agent {
    docker {
      image 'rust:latest'
      args '''--volume jenkins-data:/var/jenkins_home
--volume jenkins-docker-certs:/certs/client:ro'''
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