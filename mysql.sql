-- MySQL dump 10.13  Distrib 8.0.26, for macos11 (x86_64)
--
-- Host: 127.0.0.1    Database: blogs
-- ------------------------------------------------------
-- Server version	8.0.26

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `blog_comments`
--

DROP TABLE IF EXISTS `blog_comments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_comments` (
                                 `id` int NOT NULL AUTO_INCREMENT,
                                 `blog_id` int NOT NULL COMMENT '博客关联id',
                                 `name` varchar(100) DEFAULT NULL COMMENT '名字',
                                 `email` varchar(200) DEFAULT NULL COMMENT '邮箱',
                                 `url` varchar(500) DEFAULT NULL COMMENT '网址',
                                 `comment` text COMMENT '评论',
                                 `create_time` datetime DEFAULT NULL COMMENT '发表时间',
                                 PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=45 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客评论';
/*!40101 SET character_set_client = @saved_cs_client */;


--
-- Table structure for table `blog_config`
--

DROP TABLE IF EXISTS `blog_config`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_config` (
                               `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
                               `avatar_path` int DEFAULT NULL COMMENT '头像路径关联id',
                               `bg_path` int DEFAULT NULL COMMENT '背景地址关联id',
                               `blog_name` varchar(20) DEFAULT NULL COMMENT '博客名称',
                               `blog_brief_introduction` varchar(1024) DEFAULT NULL COMMENT '博客简介',
                               `is_use` tinyint(1) DEFAULT '0' COMMENT '是否使用',
                               PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客配置';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_config`
--

LOCK TABLES `blog_config` WRITE;
/*!40000 ALTER TABLE `blog_config` DISABLE KEYS */;
INSERT INTO `blog_config` VALUES (1,NULL,NULL,'陈年旧事。','必须记住我们学习的时间是有限的。时间有限，不只由于人生短促，更由于人事纷繁。我们就应力求把我们所有的时间用去做最有益的事情。',1);
/*!40000 ALTER TABLE `blog_config` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `blog_details`
--

DROP TABLE IF EXISTS `blog_details`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_details` (
                                `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
                                `blog_info_id` int DEFAULT NULL COMMENT '博客信息id',
                                `details` text COMMENT '博客的详细信息',
                                `create_time` datetime DEFAULT NULL COMMENT '创建时间',
                                `update_time` datetime DEFAULT NULL COMMENT '修改时间',
                                PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客详情';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `blog_files`
--

DROP TABLE IF EXISTS `blog_files`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_files` (
                              `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
                              `file_url` varchar(1024) NOT NULL COMMENT '文件路径',
                              `upload_time` datetime NOT NULL COMMENT '上传时间',
                              PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='文件信息';
/*!40101 SET character_set_client = @saved_cs_client */;


--
-- Table structure for table `blog_info`
--

DROP TABLE IF EXISTS `blog_info`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_info` (
                             `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
                             `title` varchar(256) DEFAULT NULL COMMENT '博客标题',
                             `user_account` varchar(50) DEFAULT NULL COMMENT '发表人',
                             `publish_time` datetime DEFAULT NULL COMMENT '发表时间',
                             `label_key` varchar(512) DEFAULT NULL COMMENT '博客分类关联id',
                             `read_count` int DEFAULT NULL COMMENT '阅读次数',
                             `is_publish` varchar(1) DEFAULT NULL COMMENT '是否发表 0 未发表 1 已发表',
                             `create_time` datetime DEFAULT NULL COMMENT '创建时间',
                             `update_time` datetime DEFAULT NULL COMMENT '修改时间',
                             PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=73 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客列表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_info`
--

LOCK TABLES `blog_info` WRITE;
/*!40000 ALTER TABLE `blog_info` DISABLE KEYS */;
INSERT INTO `blog_info` VALUES (20,'这是测试博客1','gxk','2021-11-27 20:45:08','js,rust',177,'1','2021-11-27 20:43:53','2021-11-27 20:43:53');
/*!40000 ALTER TABLE `blog_info` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `blog_label`
--

DROP TABLE IF EXISTS `blog_label`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_label` (
                              `id` int NOT NULL AUTO_INCREMENT COMMENT '主键',
                              `label_key` varchar(10) DEFAULT NULL COMMENT '分组标识',
                              `label_value` varchar(20) DEFAULT NULL COMMENT '分组名称',
                              `create_time` datetime DEFAULT NULL COMMENT '创建时间',
                              `update_time` datetime DEFAULT NULL COMMENT '修改时间',
                              PRIMARY KEY (`id`),
                              UNIQUE KEY `blog_label_group_key_uindex` (`label_key`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='博客分组';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_label`
--

LOCK TABLES `blog_label` WRITE;
/*!40000 ALTER TABLE `blog_label` DISABLE KEYS */;
INSERT INTO `blog_label` VALUES (1,'js','技术','2021-11-22 11:28:55','2021-11-22 11:28:58'),(2,'sh','生活','2021-11-25 18:51:45','2021-11-25 18:51:48'),(3,'rust','rust','2022-03-08 19:52:12','2022-03-08 19:52:14');
/*!40000 ALTER TABLE `blog_label` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `blog_links`
--

DROP TABLE IF EXISTS `blog_links`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `blog_links` (
                              `id` int NOT NULL AUTO_INCREMENT,
                              `link_name` varchar(100) DEFAULT NULL,
                              `link_url` varchar(100) DEFAULT NULL,
                              `flag` varchar(1) DEFAULT '0' COMMENT '是否展示 0 不展示 1 展示',
                              `create_time` datetime DEFAULT NULL COMMENT '添加时间',
                              PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=34 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='友链';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `blog_links`
--

LOCK TABLES `blog_links` WRITE;
/*!40000 ALTER TABLE `blog_links` DISABLE KEYS */;
INSERT INTO `blog_links` VALUES (1,'百度','https://www.baidu.com','1','2021-12-12 11:54:54'),(33,'你好污呀-小视频','https://www.nihaowua.com/v/','1','2021-12-12 16:35:15');
/*!40000 ALTER TABLE `blog_links` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2022-03-20 17:18:00
