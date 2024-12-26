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
   }
 }