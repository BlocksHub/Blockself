plugins {
    id("com.android.library") version "8.5.0"
    id("org.jetbrains.kotlin.android") version "2.0.0"
    id("maven-publish")
}

android {
    namespace = "com.blockshub.blockself"
    compileSdk = 34
    defaultConfig { minSdk = 24 }
    publishing { singleVariant("release") }
}

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.0")
}

publishing {
    publications {
        register<MavenPublication>("release") {
            groupId = "com.blockshub"
            artifactId = "blockself"
            version = project.findProperty("version") as String? ?: "0.0.0"
            afterEvaluate { from(components["release"]) }
        }
    }
    repositories {
        maven {
            url = uri("https://maven.pkg.github.com/BlocksHub/Blockself")
            credentials {
                username = System.getenv("GITHUB_ACTOR")
                password = System.getenv("GITHUB_TOKEN")
            }
        }
    }
}