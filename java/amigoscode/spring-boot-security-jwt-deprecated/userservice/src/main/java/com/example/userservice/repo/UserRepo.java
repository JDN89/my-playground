package com.example.userservice.repo;

import com.example.userservice.domain.AppUser;
import org.springframework.data.jpa.repository.JpaRepository;

// tell jpa the class you're going to manage and also the type of the pk
public interface UserRepo extends JpaRepository<AppUser, Long> {
    AppUser findByUsername(String username);
}
