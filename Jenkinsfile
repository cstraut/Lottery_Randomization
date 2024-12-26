#!groovy
pipeline {
    agent none
  stages {     
    stage('Rust Install') {
      agent {         
        docker {          
          image 'rust:latest'         
        }       
      }       
      steps {
        sh 'cargo clean'
      }
    }
    stage('Docker Build') {
      agent any
      steps {
        sh 'docker build -t lottery_randomization:latest .'
      }
    }
  }
}